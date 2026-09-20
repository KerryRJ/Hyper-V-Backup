use crate::infra::VirtualDiskRange;
use crate::model::{BackupId, BackupProgress, Error, VirtualMachineId};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub(crate) const CHUNK_SIZE: u64 = 4 * 1024 * 1024;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub(crate) struct ChunkId(String);

impl std::fmt::Display for ChunkId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl ChunkId {
    pub(crate) fn from_manifest_id(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct ChunkExtent {
    pub(crate) offset: u64,
    pub(crate) length: u64,
    pub(crate) chunk_id: ChunkId,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct DiskManifest {
    pub(crate) disk_id: String,
    pub(crate) logical_size: u64,
    pub(crate) chunks: Vec<ChunkExtent>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct BackupManifest {
    pub(crate) schema_version: u32,
    pub(crate) backup_id: BackupId,
    pub(crate) virtual_machine_id: VirtualMachineId,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) parent_manifest: Option<ChunkId>,
    pub(crate) disks: Vec<DiskManifest>,
    pub(crate) reference_point: ReferencePointMetadata,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct ReferencePointMetadata {
    pub(crate) path: String,
    pub(crate) instance_id: String,
    pub(crate) resilient_change_tracking_identifiers: Vec<String>,
}

pub(crate) struct BackupRepository {
    root: PathBuf,
    catalog: Connection,
}

impl BackupRepository {
    pub(crate) async fn open(root: impl Into<PathBuf>) -> Result<Self, Error> {
        let root = root.into();
        tokio::fs::create_dir_all(&root).await?;
        let catalog_path = root.join("catalog.sqlite");
        let catalog = Connection::open(catalog_path).map_err(repository_error)?;
        catalog
            .execute_batch(
                "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             CREATE TABLE IF NOT EXISTS backups (
                 backup_id TEXT PRIMARY KEY,
                 virtual_machine_id TEXT NOT NULL,
                 created_at TEXT NOT NULL,
                 started_at TEXT,
                 completed_at TEXT,
                 duration_ms INTEGER,
                 backup_kind TEXT NOT NULL,
                 status TEXT NOT NULL,
                 manifest_id TEXT NOT NULL UNIQUE,
                 parent_manifest_id TEXT
             );
             CREATE TABLE IF NOT EXISTS manifest_metadata (
                 manifest_id TEXT PRIMARY KEY,
                 schema_version INTEGER NOT NULL,
                 reference_point_path TEXT NOT NULL,
                 reference_point_instance_id TEXT NOT NULL,
                 FOREIGN KEY (manifest_id) REFERENCES backups(manifest_id)
             );
             CREATE TABLE IF NOT EXISTS reference_point_tracking_ids (
                 manifest_id TEXT NOT NULL,
                 ordinal INTEGER NOT NULL,
                 tracking_id TEXT NOT NULL,
                 PRIMARY KEY (manifest_id, ordinal),
                 FOREIGN KEY (manifest_id) REFERENCES manifest_metadata(manifest_id)
             );
             CREATE INDEX IF NOT EXISTS backups_vm_created_at
                 ON backups(virtual_machine_id, created_at DESC);
             CREATE TABLE IF NOT EXISTS disk_manifests (
                 manifest_id TEXT NOT NULL,
                 disk_id TEXT NOT NULL,
                 logical_size INTEGER NOT NULL,
                 PRIMARY KEY (manifest_id, disk_id),
                 FOREIGN KEY (manifest_id) REFERENCES backups(manifest_id)
             );
             CREATE TABLE IF NOT EXISTS disk_chunks (
                 manifest_id TEXT NOT NULL,
                 disk_id TEXT NOT NULL,
                 byte_offset INTEGER NOT NULL,
                 byte_length INTEGER NOT NULL,
                 chunk_id TEXT NOT NULL,
                 PRIMARY KEY (manifest_id, disk_id, byte_offset),
                 FOREIGN KEY (manifest_id, disk_id)
                     REFERENCES disk_manifests(manifest_id, disk_id)
             );
             CREATE INDEX IF NOT EXISTS disk_chunks_restore
                 ON disk_chunks(manifest_id, disk_id, byte_offset);",
            )
            .map_err(repository_error)?;
        ensure_backup_timing_columns(&catalog)?;
        let repository = Self { root, catalog };
        tokio::fs::create_dir_all(repository.root.join("objects")).await?;
        Ok(repository)
    }

    pub(crate) async fn store_full_disk(&self, path: &Path, disk_id: String, logical_size: u64, completed_bytes: &mut u64, total_bytes: u64, progress_sender: Option<&tokio::sync::watch::Sender<BackupProgress>>) -> Result<DiskManifest, Error> {
        let mut source = tokio::fs::File::open(path).await?;
        let mut chunks = Vec::new();
        let mut offset = 0;
        while offset < logical_size {
            let length = CHUNK_SIZE.min(logical_size - offset) as usize;
            let mut bytes = vec![0; length];
            source.seek(std::io::SeekFrom::Start(offset)).await?;
            source.read_exact(&mut bytes).await?;
            let chunk_id = self.put_chunk(&bytes).await?;
            chunks.push(ChunkExtent { offset, length: length as u64, chunk_id });
            offset += length as u64;
            *completed_bytes += length as u64;
            send_progress(progress_sender, *completed_bytes, total_bytes);
        }
        Ok(DiskManifest { disk_id, logical_size, chunks })
    }

    pub(crate) async fn store_incremental_disk(&self, path: &Path, disk_id: String, logical_size: u64, parent: &DiskManifest, changed_ranges: &[VirtualDiskRange], completed_bytes: &mut u64, total_bytes: u64, progress_sender: Option<&tokio::sync::watch::Sender<BackupProgress>>) -> Result<DiskManifest, Error> {
        if parent.disk_id != disk_id || parent.logical_size != logical_size {
            return Err(Error::InvalidBackupRequest("differential backup base does not match the current virtual disk"));
        }
        let mut source = tokio::fs::File::open(path).await?;
        let mut chunks = Vec::with_capacity(parent.chunks.len());
        let mut offset = 0;
        while offset < logical_size {
            let length = CHUNK_SIZE.min(logical_size - offset);
            let parent_chunk = parent.chunks.iter().find(|chunk| chunk.offset == offset && chunk.length == length).ok_or(Error::InvalidBackupRequest("differential backup base has incomplete disk chunks"))?;
            let affected = changed_ranges.iter().any(|range| range.byte_offset < offset + length && offset < range.byte_offset + range.byte_length);
            let chunk_id = if affected {
                let mut bytes = vec![0; length as usize];
                source.seek(std::io::SeekFrom::Start(offset)).await?;
                source.read_exact(&mut bytes).await?;
                self.put_chunk(&bytes).await?
            } else {
                parent_chunk.chunk_id.clone()
            };
            chunks.push(ChunkExtent { offset, length, chunk_id });
            offset += length;
            *completed_bytes += length;
            send_progress(progress_sender, *completed_bytes, total_bytes);
        }
        Ok(DiskManifest { disk_id, logical_size, chunks })
    }

    pub(crate) async fn read_manifest(&self, manifest_id: &str) -> Result<BackupManifest, Error> {
        if manifest_id.len() != 64 || !manifest_id.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(Error::InvalidBackupRequest("manifest id must be a SHA-256 hex value"));
        }
        let manifest = self
            .catalog
            .query_row(
                "SELECT b.backup_id, b.virtual_machine_id, b.created_at, b.parent_manifest_id,
                        m.schema_version, m.reference_point_path, m.reference_point_instance_id
                 FROM backups b
                 JOIN manifest_metadata m ON m.manifest_id = b.manifest_id
                 WHERE b.manifest_id = ?1",
                params![manifest_id],
                |row| {
                    let backup_id = row.get::<_, String>(0)?.parse::<uuid::Uuid>().map(BackupId::from).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
                    let virtual_machine_id = VirtualMachineId::parse_str(&row.get::<_, String>(1)?).map_err(|error| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(error)))?;
                    let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                        .map(|value| value.with_timezone(&Utc))
                        .map_err(|error| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(error)))?;
                    let schema_version = u32::try_from(row.get::<_, i64>(4)?).map_err(|error| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Integer, Box::new(error)))?;
                    Ok((backup_id, virtual_machine_id, created_at, row.get::<_, Option<String>>(3)?.map(ChunkId), schema_version, row.get::<_, String>(5)?, row.get::<_, String>(6)?))
                },
            )
            .optional()
            .map_err(repository_error)?;

        let Some((backup_id, virtual_machine_id, created_at, parent_manifest, schema_version, reference_point_path, reference_point_instance_id)) = manifest else {
            return self.read_legacy_manifest(manifest_id).await;
        };
        let mut disks = Vec::new();
        let mut disk_statement = self.catalog.prepare("SELECT disk_id, logical_size FROM disk_manifests WHERE manifest_id = ?1 ORDER BY disk_id").map_err(repository_error)?;
        let disk_rows = disk_statement
            .query_map(params![manifest_id], |row| {
                let logical_size = u64::try_from(row.get::<_, i64>(1)?).map_err(|error| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Integer, Box::new(error)))?;
                Ok((row.get::<_, String>(0)?, logical_size))
            })
            .map_err(repository_error)?;
        for disk_row in disk_rows {
            let (disk_id, logical_size) = disk_row.map_err(repository_error)?;
            let mut chunk_statement = self.catalog.prepare("SELECT byte_offset, byte_length, chunk_id FROM disk_chunks WHERE manifest_id = ?1 AND disk_id = ?2 ORDER BY byte_offset").map_err(repository_error)?;
            let chunk_rows = chunk_statement
                .query_map(params![manifest_id, disk_id], |row| {
                    let offset = u64::try_from(row.get::<_, i64>(0)?).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Integer, Box::new(error)))?;
                    let length = u64::try_from(row.get::<_, i64>(1)?).map_err(|error| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Integer, Box::new(error)))?;
                    Ok(ChunkExtent { offset, length, chunk_id: ChunkId(row.get(2)?) })
                })
                .map_err(repository_error)?;
            let chunks = chunk_rows.map(|row| row.map_err(repository_error)).collect::<Result<Vec<_>, _>>()?;
            disks.push(DiskManifest { disk_id, logical_size, chunks });
        }
        let mut tracking_statement = self.catalog.prepare("SELECT tracking_id FROM reference_point_tracking_ids WHERE manifest_id = ?1 ORDER BY ordinal").map_err(repository_error)?;
        let resilient_change_tracking_identifiers = tracking_statement.query_map(params![manifest_id], |row| row.get(0)).map_err(repository_error)?.map(|row| row.map_err(repository_error)).collect::<Result<Vec<_>, _>>()?;
        Ok(BackupManifest {
            schema_version,
            backup_id,
            virtual_machine_id,
            created_at,
            parent_manifest,
            disks,
            reference_point: ReferencePointMetadata {
                path: reference_point_path,
                instance_id: reference_point_instance_id,
                resilient_change_tracking_identifiers,
            },
        })
    }

    async fn read_legacy_manifest(&self, manifest_id: &str) -> Result<BackupManifest, Error> {
        let path = self.manifest_path(&ChunkId(manifest_id.to_owned()));
        let bytes = tokio::fs::read(path).await.map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => Error::Repository(format!("manifest not found: {manifest_id}")),
            _ => error.into(),
        })?;
        serde_json::from_slice(&bytes).map_err(|error| Error::Repository(error.to_string()))
    }

    pub(crate) async fn latest_backup_before(&self, virtual_machine_id: VirtualMachineId, point_in_time: DateTime<Utc>) -> Result<Option<BackupManifest>, Error> {
        let manifest_id = self
            .catalog
            .query_row(
                "SELECT manifest_id
                 FROM backups
                 WHERE virtual_machine_id = ?1
                   AND created_at <= ?2
                   AND status = 'complete'
                 ORDER BY created_at DESC
                 LIMIT 1",
                params![virtual_machine_id.to_string(), point_in_time.to_rfc3339()],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(repository_error)?;
        match manifest_id {
            Some(id) => self.read_manifest(&id).await.map(Some),
            None => Ok(None),
        }
    }

    pub(crate) async fn write_manifest(&self, manifest: &BackupManifest, started_at: DateTime<Utc>, completed_at: DateTime<Utc>, duration_ms: u64) -> Result<ChunkId, Error> {
        let bytes = serde_json::to_vec_pretty(manifest).map_err(|error| Error::Repository(error.to_string()))?;
        let manifest_id = hash_bytes(&bytes);
        let exists = self.catalog.query_row("SELECT 1 FROM manifest_metadata WHERE manifest_id = ?1", params![manifest_id.to_string()], |_| Ok(())).optional().map_err(repository_error)?.is_some();
        if exists {
            return Ok(manifest_id);
        }
        self.index_manifest(&manifest_id, manifest, started_at, completed_at, duration_ms)?;
        Ok(manifest_id)
    }

    fn index_manifest(&self, manifest_id: &ChunkId, manifest: &BackupManifest, started_at: DateTime<Utc>, completed_at: DateTime<Utc>, duration_ms: u64) -> Result<(), Error> {
        let transaction = self.catalog.unchecked_transaction().map_err(repository_error)?;
        let backup_kind = if manifest.parent_manifest.is_some() { "incremental" } else { "full" };
        transaction
            .execute(
                "INSERT OR IGNORE INTO backups
                      (backup_id, virtual_machine_id, created_at, started_at, completed_at, duration_ms, backup_kind, status, manifest_id, parent_manifest_id)
                      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'complete', ?8, ?9)",
                params![
                    manifest.backup_id.to_string(),
                    manifest.virtual_machine_id.to_string(),
                    manifest.created_at.to_rfc3339(),
                    started_at.to_rfc3339(),
                    completed_at.to_rfc3339(),
                    sqlite_integer(duration_ms)?,
                    backup_kind,
                    manifest_id.to_string(),
                    manifest.parent_manifest.as_ref().map(ToString::to_string),
                ],
            )
            .map_err(repository_error)?;
        transaction
            .execute(
                "INSERT OR IGNORE INTO manifest_metadata
                 (manifest_id, schema_version, reference_point_path, reference_point_instance_id)
                 VALUES (?1, ?2, ?3, ?4)",
                params![manifest_id.to_string(), sqlite_integer(u64::from(manifest.schema_version))?, manifest.reference_point.path, manifest.reference_point.instance_id,],
            )
            .map_err(repository_error)?;
        for (ordinal, tracking_id) in manifest.reference_point.resilient_change_tracking_identifiers.iter().enumerate() {
            transaction
                .execute(
                    "INSERT OR IGNORE INTO reference_point_tracking_ids (manifest_id, ordinal, tracking_id)
                     VALUES (?1, ?2, ?3)",
                    params![manifest_id.to_string(), sqlite_integer(ordinal as u64)?, tracking_id],
                )
                .map_err(repository_error)?;
        }
        for disk in &manifest.disks {
            transaction
                .execute(
                    "INSERT OR IGNORE INTO disk_manifests (manifest_id, disk_id, logical_size)
                     VALUES (?1, ?2, ?3)",
                    params![manifest_id.to_string(), disk.disk_id, sqlite_integer(disk.logical_size)?],
                )
                .map_err(repository_error)?;
            for chunk in &disk.chunks {
                transaction
                    .execute(
                        "INSERT OR IGNORE INTO disk_chunks
                         (manifest_id, disk_id, byte_offset, byte_length, chunk_id)
                         VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![manifest_id.to_string(), disk.disk_id, sqlite_integer(chunk.offset)?, sqlite_integer(chunk.length)?, chunk.chunk_id.to_string(),],
                    )
                    .map_err(repository_error)?;
            }
        }
        transaction.commit().map_err(repository_error)
    }

    async fn put_chunk(&self, bytes: &[u8]) -> Result<ChunkId, Error> {
        let chunk_id = hash_bytes(bytes);
        let path = self.object_path(&chunk_id);
        if tokio::fs::try_exists(&path).await? {
            return Ok(chunk_id);
        }
        if tokio::fs::try_exists(self.legacy_object_path(&chunk_id)).await? {
            return Ok(chunk_id);
        }
        let compressed = zstd::encode_all(bytes, 3).map_err(|error| Error::Repository(format!("failed to compress chunk {chunk_id}: {error}")))?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let temporary_path = path.with_extension("tmp");
        let mut file = tokio::fs::File::create(&temporary_path).await?;
        file.write_all(&compressed).await?;
        file.flush().await?;
        drop(file);
        match tokio::fs::rename(&temporary_path, path).await {
            Ok(()) => Ok(chunk_id),
            Err(_error) if tokio::fs::try_exists(self.object_path(&chunk_id)).await? => Ok(chunk_id),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) async fn read_chunk(&self, chunk_id: &ChunkId) -> Result<Vec<u8>, Error> {
        let path = self.object_path(chunk_id);
        let bytes = match tokio::fs::read(&path).await {
            Ok(bytes) => zstd::decode_all(bytes.as_slice()).map_err(|error| Error::Repository(format!("failed to decompress chunk {chunk_id}: {error}")))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => tokio::fs::read(self.legacy_object_path(chunk_id)).await?,
            Err(error) => return Err(error.into()),
        };
        if hash_bytes(&bytes) != *chunk_id {
            return Err(Error::Repository(format!("chunk {chunk_id} failed integrity validation")));
        }
        Ok(bytes)
    }

    fn object_path(&self, chunk_id: &ChunkId) -> PathBuf {
        self.root.join("objects").join(&chunk_id.0[..2]).join(format!("{}.chunk.zst", chunk_id.0))
    }

    fn legacy_object_path(&self, chunk_id: &ChunkId) -> PathBuf {
        self.root.join("objects").join(&chunk_id.0[..2]).join(format!("{}.chunk", chunk_id.0))
    }

    fn manifest_path(&self, manifest_id: &ChunkId) -> PathBuf {
        self.root.join("manifests").join(format!("{}.json", manifest_id.0))
    }
}

fn hash_bytes(bytes: &[u8]) -> ChunkId {
    let digest = Sha256::digest(bytes);
    ChunkId(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn sqlite_integer(value: u64) -> Result<i64, Error> {
    i64::try_from(value).map_err(|_| Error::Repository("value exceeds SQLite integer range".into()))
}

fn repository_error(error: rusqlite::Error) -> Error {
    Error::Repository(error.to_string())
}

fn ensure_backup_timing_columns(catalog: &Connection) -> Result<(), Error> {
    let mut statement = catalog.prepare("PRAGMA table_info(backups)").map_err(repository_error)?;
    let columns = statement.query_map([], |row| row.get::<_, String>(1)).map_err(repository_error)?.map(|row| row.map_err(repository_error)).collect::<Result<Vec<_>, _>>()?;
    for (name, definition) in [("started_at", "TEXT"), ("completed_at", "TEXT"), ("duration_ms", "INTEGER")] {
        if !columns.iter().any(|column| column == name) {
            catalog.execute(&format!("ALTER TABLE backups ADD COLUMN {name} {definition}"), []).map_err(repository_error)?;
        }
    }
    Ok(())
}

fn send_progress(sender: Option<&tokio::sync::watch::Sender<BackupProgress>>, completed_bytes: u64, total_bytes: u64) {
    if let Some(sender) = sender {
        let _ = sender.send(BackupProgress::new(completed_bytes, total_bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::VirtualDiskRange;

    #[tokio::test]
    async fn stores_duplicate_chunks_once_and_round_trips_manifest() {
        let root = std::env::temp_dir().join(format!("backup-repository-{}", uuid::Uuid::new_v4()));
        let repository = BackupRepository::open(&root).await.expect("repository should open");
        let first = repository.put_chunk(b"same content").await.expect("first chunk should store");
        let second = repository.put_chunk(b"same content").await.expect("duplicate chunk should store");
        assert_eq!(first, second);
        assert_eq!(repository.read_chunk(&first).await.expect("chunk should round-trip"), b"same content");

        let manifest = BackupManifest {
            schema_version: 1,
            backup_id: BackupId::new_v4(),
            virtual_machine_id: VirtualMachineId::from(uuid::Uuid::new_v4()),
            created_at: Utc::now(),
            parent_manifest: None,
            disks: Vec::new(),
            reference_point: ReferencePointMetadata {
                path: "reference-point".into(),
                instance_id: "instance".into(),
                resilient_change_tracking_identifiers: Vec::new(),
            },
        };
        let started_at = Utc::now();
        let completed_at = started_at + chrono::Duration::milliseconds(123);
        let manifest_id = repository.write_manifest(&manifest, started_at, completed_at, 123).await.expect("manifest should store");
        let restored = repository.read_manifest(&manifest_id.to_string()).await.expect("manifest should be readable");
        assert_eq!(restored.backup_id, manifest.backup_id);
        let timing = repository
            .catalog
            .query_row("SELECT started_at, completed_at, duration_ms FROM backups WHERE manifest_id = ?1", params![manifest_id.to_string()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, i64>(2)?))
            })
            .expect("backup timing should be persisted");
        assert_eq!(timing.0, started_at.to_rfc3339());
        assert_eq!(timing.1, completed_at.to_rfc3339());
        assert_eq!(timing.2, 123);
        let mut objects = tokio::fs::read_dir(root.join("objects").join(&first.0[..2])).await.expect("object directory should exist");
        assert!(objects.next_entry().await.expect("object directory should be readable").is_some());
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn compresses_unique_chunks_before_storage() {
        let root = std::env::temp_dir().join(format!("backup-repository-{}", uuid::Uuid::new_v4()));
        let repository = BackupRepository::open(&root).await.expect("repository should open");
        let source = vec![b'a'; CHUNK_SIZE as usize];
        let chunk_id = repository.put_chunk(&source).await.expect("chunk should store");
        let stored_size = tokio::fs::metadata(repository.object_path(&chunk_id)).await.expect("compressed object should exist").len();

        assert!(stored_size < source.len() as u64);
        assert_eq!(repository.read_chunk(&chunk_id).await.expect("compressed chunk should round-trip"), source);
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn incremental_storage_reuses_unchanged_aligned_chunks() {
        let root = std::env::temp_dir().join(format!("backup-repository-{}", uuid::Uuid::new_v4()));
        let repository = BackupRepository::open(&root).await.expect("repository should open");
        let source_path = root.join("disk.bin");
        let original = vec![b'a'; (CHUNK_SIZE * 2) as usize];
        tokio::fs::write(&source_path, &original).await.expect("source should be written");
        let mut completed_bytes = 0;
        let parent = repository.store_full_disk(&source_path, "disk-1".into(), original.len() as u64, &mut completed_bytes, original.len() as u64, None).await.expect("parent disk should store");

        let mut changed = original;
        changed[CHUNK_SIZE as usize] = b'b';
        tokio::fs::write(&source_path, &changed).await.expect("changed source should be written");
        let child = repository
            .store_incremental_disk(&source_path, "disk-1".into(), changed.len() as u64, &parent, &[VirtualDiskRange { byte_offset: CHUNK_SIZE, byte_length: 1 }], &mut completed_bytes, changed.len() as u64, None)
            .await
            .expect("child disk should store");

        assert_eq!(child.chunks[0].chunk_id, parent.chunks[0].chunk_id);
        assert_ne!(child.chunks[1].chunk_id, parent.chunks[1].chunk_id);
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn point_in_time_lookup_returns_latest_complete_manifest() {
        let root = std::env::temp_dir().join(format!("backup-repository-{}", uuid::Uuid::new_v4()));
        let repository = BackupRepository::open(&root).await.expect("repository should open");
        let virtual_machine_id = VirtualMachineId::from(uuid::Uuid::new_v4());
        let first_time = Utc::now();
        let second_time = first_time + chrono::Duration::minutes(5);
        let manifest = |created_at, backup_id| BackupManifest {
            schema_version: 1,
            backup_id,
            virtual_machine_id,
            created_at,
            parent_manifest: None,
            disks: Vec::new(),
            reference_point: ReferencePointMetadata {
                path: "reference-point".into(),
                instance_id: "instance".into(),
                resilient_change_tracking_identifiers: Vec::new(),
            },
        };
        let first = manifest(first_time, BackupId::new_v4());
        let second = manifest(second_time, BackupId::new_v4());
        repository.write_manifest(&first, first_time, first_time, 10).await.expect("first manifest should store");
        repository.write_manifest(&second, second_time, second_time, 20).await.expect("second manifest should store");

        let selected = repository.latest_backup_before(virtual_machine_id, second_time + chrono::Duration::seconds(1)).await.expect("point-in-time lookup should work").expect("a backup should be found");
        assert_eq!(selected.backup_id, second.backup_id);
        let selected = repository.latest_backup_before(virtual_machine_id, first_time + chrono::Duration::seconds(1)).await.expect("point-in-time lookup should work").expect("a backup should be found");
        assert_eq!(selected.backup_id, first.backup_id);
        let _ = tokio::fs::remove_dir_all(root).await;
    }
}
