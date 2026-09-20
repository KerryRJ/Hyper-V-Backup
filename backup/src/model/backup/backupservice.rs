use super::*;
use crate::infra::*;
use crate::model::*;
use chrono::Utc;
use std::ffi::OsStr;
use std::path::Path;

pub struct BackupService {
    snapshot_service: VirtualSystemSnapshotService,
    management_service: VirtualSystemManagementService,
    image_management_service: ImageManagementService,
}

impl BackupService {
    // pub async fn backup(&self, request: BackupRequest) -> Result<BackupResult, Error> {
    //     let virtual_machine = request.virtual_machine;
    //     let virtual_machine_name = virtual_machine.element_name.clone();
    //     let is_incremental = request.differential_backup_base.is_some();
    //     log::info!("Starting {} backup for virtual machine name {}", if is_incremental { "incremental" } else { "full" }, virtual_machine_name);
    //     let snapshot_settings = SnapshotSettings {
    //         properties: vec![
    //             SnapshotProperty {
    //                 name: "ConsistencyLevel".into(),
    //                 value: SnapshotPropertyValue::Uint8(u8::from(request.crash_consistency)),
    //             },
    //             SnapshotProperty {
    //                 name: "IgnoreNonSnapshottableDisks".into(),
    //                 value: SnapshotPropertyValue::Boolean(true),
    //             },
    //         ],
    //     };
    //     let snapshot = self.snapshot_service.create(&virtual_machine, Some((&snapshot_settings).into()), request.snapshot_type, None).await?;
    //     log::debug!("Created backup snapshot {snapshot:#?} for virtual machine name {virtual_machine_name}");
    //     let mut export_settings = ExportSettings {
    //         properties: vec![
    //             ExportProperty {
    //                 name: "CaptureLiveState".into(),
    //                 value: ExportPropertyValue::Uint8(0),
    //             },
    //             ExportProperty {
    //                 name: "CopySnapshotConfiguration".into(),
    //                 value: ExportPropertyValue::Uint8(CopySnapshotConfiguration::ExportOneSnapshotForBackup as u8),
    //             },
    //             ExportProperty {
    //                 name: "CopyVmRuntimeInformation".into(),
    //                 value: ExportPropertyValue::Boolean(false),
    //             },
    //             ExportProperty {
    //                 name: "CopyVmStorage".into(),
    //                 value: ExportPropertyValue::Boolean(true),
    //             },
    //             ExportProperty {
    //                 name: "CreateVmExportSubdirectory".into(),
    //                 value: ExportPropertyValue::Boolean(false),
    //             },
    //         ],
    //     };
    //     export_settings.properties.push(ExportProperty {
    //         name: "SnapshotVirtualSystem".into(),
    //         value: ExportPropertyValue::String(snapshot.path.as_str().to_owned()),
    //     });
    //     let export_prefix = if request.differential_backup_base.is_some() { "i" } else { "f" };
    //     let differential_backup_base = request.differential_backup_base.as_ref();
    //     if let Some(reference_point) = differential_backup_base {
    //         log::debug!("Using differential backup base {}", reference_point.path.as_str());

    //         let current_reference_points = virtual_machine.get_reference_points().await;
    //         log::debug!("Current reference points on virtual machine {}: {current_reference_points:#?}", virtual_machine_name);

    //         export_settings.properties.push(ExportProperty {
    //             name: "DifferentialBackupBase".into(),
    //             value: ExportPropertyValue::String(reference_point.path.as_str().to_owned()),
    //         });
    //     }
    //     log::debug!("Export settings: {export_settings:#?}");
    //     let export_directory = request.destination.join(format!("{export_prefix}-{}", Local::now().format("%Y%m%d-%H%M%S")));
    //     log::info!("Exporting backup for virtual machine name {} to {}", virtual_machine_name, export_directory.display(),);
    //     let result = self.management_service.export_system_definition(virtual_machine.clone(), export_directory.clone(), Some(VirtualSystemExportSettingDataIn::from(&export_settings))).await;
    //     match result {
    //         Ok(_) => {
    //             log::debug!("Export completed for virtual machine name {}; converting snapshot {} to a reference point", virtual_machine_name, snapshot.path.as_str());
    //             let reference_point_settings = ReferencePointSettings {
    //                 properties: vec![ReferencePointProperty {
    //                     name: "ConsistencyLevel".into(),
    //                     value: ReferencePointPropertyValue::Uint8(u8::from(request.crash_consistency)),
    //                 }],
    //             };
    //             let reference_point = self.snapshot_service.convert_to_reference_point(snapshot, Some((&reference_point_settings).into()), None).await?;
    //             log::info!("Completed {} backup for virtual machine name {} with reference point {}", if is_incremental { "incremental" } else { "full" }, virtual_machine_name, reference_point.path.as_str());
    //             Ok(BackupResult {
    //                 backup_id: BackupId::new_v4(),
    //                 virtual_machine: virtual_machine,
    //                 destination: request.destination,
    //                 reference_point: reference_point,
    //                 completed_at: Utc::now(),
    //             })
    //         }
    //         Err(error) => {
    //             log::error!("Failed to export {} backup for virtual machine name {}: {}", if is_incremental { "incremental" } else { "full" }, virtual_machine_name, error);
    //             log::debug!("Destroying failed backup snapshot {}", snapshot.path.as_str());
    //             self.snapshot_service.destroy(snapshot).await?;
    //             Err(error.into())
    //         }
    //     }
    // }

    pub async fn backup(&self, request: BackupRequest) -> Result<BackupResult, Error> {
        let started_at = Utc::now();
        let elapsed = std::time::Instant::now();
        let virtual_machine = request.virtual_machine;
        let virtual_machine_name = virtual_machine.element_name.clone();
        let is_incremental = request.differential_backup_base.is_some();
        log::info!("Starting {} backup for virtual machine name {}", if is_incremental { "incremental" } else { "full" }, virtual_machine_name);
        let snapshot_settings = SnapshotSettings {
            properties: vec![
                SnapshotProperty {
                    name: "ConsistencyLevel".into(),
                    value: SnapshotPropertyValue::Uint8(u8::from(request.crash_consistency)),
                },
                SnapshotProperty {
                    name: "IgnoreNonSnapshottableDisks".into(),
                    value: SnapshotPropertyValue::Boolean(true),
                },
            ],
        };
        let snapshot = self.snapshot_service.create(&virtual_machine, Some((&snapshot_settings).into()), request.snapshot_type, None).await?;
        log::debug!("Created backup snapshot {snapshot:#?} for virtual machine name {virtual_machine_name}");
        let result = async {
            let repository = BackupRepository::open(&request.destination).await?;
            let mut disk_manifests = Vec::new();
            let parent_manifest = match request.differential_backup_base_manifest_id.as_deref() {
                Some(manifest_id) => Some(repository.read_manifest(manifest_id).await?),
                None if is_incremental => return Err(Error::InvalidBackupRequest("incremental backup requires a parent manifest id")),
                None => None,
            };
            let storage_allocation_setting_data = self.snapshot_service.storage_allocation_setting_data(&snapshot).await?;
            log::debug!("StorageAllocationSettingData returned for snapshot {}: {storage_allocation_setting_data:#?}", snapshot.path.as_str(),);
            let mut virtual_hard_disk_settings = Vec::new();
            for storage_allocation in storage_allocation_setting_data {
                if !storage_allocation.resource_sub_type.contains("Virtual Hard Disk") {
                    continue;
                }
                for host_resource in &storage_allocation.host_resource {
                    let path = host_resource.to_string_lossy().into_owned();
                    let virtual_hard_disk = self.image_management_service.get_virtual_hard_disk_setting_data(&path).await?;
                    virtual_hard_disk_settings.push(virtual_hard_disk);
                }
            }
            log::debug!("Virtual hard disk settings returned for snapshot {}: {virtual_hard_disk_settings:#?}", snapshot.path.as_str());
            let total_bytes = virtual_hard_disk_settings.iter().map(|setting| std::fs::metadata(setting.path.as_str()).map(|metadata| metadata.len())).collect::<Result<Vec<_>, _>>()?.into_iter().sum();
            let mut completed_bytes = 0;
            if let Some(progress_sender) = request.progress_sender.as_ref() {
                let _ = progress_sender.send(BackupProgress::new(0, total_bytes));
            }
            let differential_backup_base = request.differential_backup_base.as_ref();
            for (index, virtual_hard_disk_setting) in virtual_hard_disk_settings.iter().enumerate() {
                log::debug!("Opening virtual hard disk {}", virtual_hard_disk_setting.path.as_str());
                let virtual_disk = VirtualDisk::open(virtual_hard_disk_setting).await?;
                match differential_backup_base {
                    None => {
                        log::info!("No differential backup base; storing full virtual disk {} as repository chunks", virtual_hard_disk_setting.path.as_str());
                        let disk_size = std::fs::metadata(virtual_hard_disk_setting.path.as_str())?.len();
                        let virtual_disk_path = Path::new(virtual_hard_disk_setting.path.as_str());
                        let disk_manifest = repository.store_full_disk(virtual_disk_path, virtual_hard_disk_setting.virtual_disk_id().to_string(), disk_size, &mut completed_bytes, total_bytes, request.progress_sender.as_ref()).await?;
                        disk_manifests.push(disk_manifest);
                    }
                    Some(differential_backup_base) => {
                        let parent_manifest = parent_manifest.as_ref().ok_or(Error::InvalidBackupRequest("incremental backup requires a parent manifest"))?;
                        let parent_disk = parent_manifest
                            .disks
                            .iter()
                            .find(|disk| disk.disk_id == virtual_hard_disk_setting.virtual_disk_id().to_string())
                            .ok_or(Error::InvalidBackupRequest("parent manifest is missing a virtual disk"))?;
                        let change_tracking_id = differential_backup_base.resilient_change_tracking_identifiers.get(index).ok_or(Error::InvalidBackupRequest("reference point is missing a disk change-tracking identifier"))?;
                        let change_tracking_id = change_tracking_id.to_string();
                        let changed_ranges = virtual_disk.virtual_disk_changes(OsStr::new(&change_tracking_id), u64::MAX)?;
                        let disk_size = std::fs::metadata(virtual_hard_disk_setting.path.as_str())?.len();
                        let disk_manifest = repository
                            .store_incremental_disk(
                                Path::new(virtual_hard_disk_setting.path.as_str()),
                                virtual_hard_disk_setting.virtual_disk_id().to_string(),
                                disk_size,
                                parent_disk,
                                changed_ranges,
                                &mut completed_bytes,
                                total_bytes,
                                request.progress_sender.as_ref(),
                            )
                            .await?;
                        disk_manifests.push(disk_manifest);
                    }
                }
            }
            // TODO: Seal and Transition to Reference Point
            let reference_point_settings = ReferencePointSettings {
                properties: vec![ReferencePointProperty {
                    name: "ConsistencyLevel".into(),
                    value: ReferencePointPropertyValue::Uint8(u8::from(request.crash_consistency)),
                }],
            };
            let reference_point = self.snapshot_service.convert_to_reference_point(snapshot.clone(), Some((&reference_point_settings).into()), None).await?;
            let backup_id = BackupId::new_v4();
            let manifest = BackupManifest {
                schema_version: 1,
                backup_id,
                virtual_machine_id: virtual_machine.name,
                created_at: Utc::now(),
                parent_manifest: request.differential_backup_base_manifest_id.clone().map(super::repository::ChunkId::from_manifest_id),
                disks: disk_manifests,
                reference_point: ReferencePointMetadata {
                    path: reference_point.path.as_str().to_owned(),
                    instance_id: reference_point.instance_id_string(),
                    resilient_change_tracking_identifiers: reference_point.resilient_change_tracking_identifier_strings(),
                },
            };
            let completed_at = Utc::now();
            let duration = elapsed.elapsed();
            let duration_ms = u64::try_from(duration.as_millis()).unwrap_or(u64::MAX);
            let manifest_id = repository.write_manifest(&manifest, started_at, completed_at, duration_ms).await?;
            if let Some(progress_sender) = request.progress_sender.as_ref() {
                let _ = progress_sender.send(BackupProgress::new(total_bytes, total_bytes));
            }
            log::info!(
                "Completed {} backup for virtual machine name {} in {:?} ({} ms) with reference point {}",
                if is_incremental { "incremental" } else { "full" },
                virtual_machine_name,
                duration,
                duration_ms,
                reference_point.path.as_str()
            );
            Ok::<BackupResult, Error>(BackupResult {
                backup_id,
                virtual_machine: virtual_machine,
                destination: request.destination,
                reference_point: reference_point,
                manifest_id: manifest_id.to_string(),
                completed_at,
            })
        }
        .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => {
                if let Err(cleanup_error) = self.snapshot_service.destroy(snapshot).await {
                    log::error!("Failed to remove recovery snapshot after backup failure: {cleanup_error}");
                }
                Err(error)
            }
        }
    }
}

#[cfg(all(test, windows))]
mod tests {
    use std::path::PathBuf;
    use tokio::sync::watch;

    use super::*;

    async fn backup_with_progress(service: &BackupService, request: BackupRequest, label: &str) -> Result<BackupResult, Error> {
        let (progress_sender, mut progress_receiver) = watch::channel(BackupProgress { completed_bytes: 0, total_bytes: 0, percent: 0 });
        let request = BackupRequest { progress_sender: Some(progress_sender), ..request };
        let backup = service.backup(request);
        tokio::pin!(backup);
        let mut last_percent = u8::MAX;

        loop {
            tokio::select! {
                result = &mut backup => {
                    let progress = progress_receiver.borrow().clone();
                    println!("{label} progress: {}% ({}/{} bytes)", progress.percent, progress.completed_bytes, progress.total_bytes);
                    return result;
                }
                changed = progress_receiver.changed() => {
                    if changed.is_err() {
                        continue;
                    }
                    let progress = progress_receiver.borrow().clone();
                    if progress.percent != last_percent {
                        println!("{label} progress: {}% ({}/{} bytes)", progress.percent, progress.completed_bytes, progress.total_bytes);
                        last_percent = progress.percent;
                    }
                }
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM and writable export directory"]
    async fn full_then_incremental_backup_exports_vm_and_returns_results() {
        let _ = env_logger::try_init();
        let virtual_machine_name = std::env::var("HYPER_V_VM").expect("HYPER_V_VM must be set");
        let destination = PathBuf::from(std::env::var("HYPER_V_BACKUP_DESTINATION").expect("HYPER_V_BACKUP_DESTINATION must be set"));
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let virtual_machine = Host::new().expect("Hyper-V host should be available").get_virtual_machine_by_name(&virtual_machine_name).await.expect("Hyper-V VM should be available");
        let virtual_machine_id = virtual_machine.name;
        let reference_point_service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        let service = BackupService {
            snapshot_service: VirtualSystemSnapshotService::new(connection.clone()).expect("Hyper-V snapshot service should be available"),
            management_service: VirtualSystemManagementService::new(connection.clone()).expect("Hyper-V management service should be available"),
            image_management_service: ImageManagementService::new(connection.clone()).expect("Hyper-V image management service should be available"),
        };
        let backup_request = BackupRequest {
            virtual_machine: virtual_machine.clone(),
            snapshot_type: SnapshotType::VendorSpecific(32768),
            crash_consistency: ConsistencyLevel::Crash,
            destination: destination.clone(),
            differential_backup_base: None,
            differential_backup_base_manifest_id: None,
            progress_sender: None,
        };
        let full_result = backup_with_progress(&service, backup_request, "full backup").await.expect("full backup should complete");

        assert_eq!(full_result.virtual_machine.name, virtual_machine_id);
        assert_eq!(full_result.destination, destination);
        assert!(!full_result.backup_id.to_string().is_empty());
        assert!(!full_result.reference_point.path.to_string().is_empty());
        let full_reference_point = full_result.reference_point;

        let incremental_destination = destination;
        let incremental_result = backup_with_progress(
            &service,
            BackupRequest {
                virtual_machine,
                snapshot_type: SnapshotType::VendorSpecific(32768),
                crash_consistency: ConsistencyLevel::Crash,
                destination: incremental_destination.clone(),
                differential_backup_base: Some(full_reference_point.clone()),
                differential_backup_base_manifest_id: Some(full_result.manifest_id.clone()),
                progress_sender: None,
            },
            "incremental backup",
        )
        .await
        .expect("incremental backup should complete");

        assert_eq!(incremental_result.virtual_machine.name, virtual_machine_id);
        assert_eq!(incremental_result.destination, incremental_destination);
        assert!(!incremental_result.backup_id.to_string().is_empty());
        assert!(!incremental_result.reference_point.path.to_string().is_empty());

        reference_point_service.destroy(&full_reference_point).await.expect("full backup reference point should be destroyed");
        reference_point_service.destroy(&incremental_result.reference_point).await.expect("incremental backup reference point should be destroyed");
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM and writable export directory"]
    async fn backup_new_creates_reference_point_and_reads_storage_allocations() {
        let _ = env_logger::try_init();
        let virtual_machine_name = std::env::var("HYPER_V_VM").expect("HYPER_V_VM must be set");
        let destination = PathBuf::from(std::env::var("HYPER_V_BACKUP_DESTINATION").expect("HYPER_V_BACKUP_DESTINATION must be set"));
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let virtual_machine = Host::new().expect("Hyper-V host should be available").get_virtual_machine_by_name(&virtual_machine_name).await.expect("Hyper-V VM should be available");
        let virtual_machine_id = virtual_machine.name;
        let reference_point_service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        let service = BackupService {
            snapshot_service: VirtualSystemSnapshotService::new(connection.clone()).expect("Hyper-V snapshot service should be available"),
            management_service: VirtualSystemManagementService::new(connection.clone()).expect("Hyper-V management service should be available"),
            image_management_service: ImageManagementService::new(connection.clone()).expect("Hyper-V image management service should be available"),
        };

        let result = backup_with_progress(
            &service,
            BackupRequest {
                virtual_machine,
                snapshot_type: SnapshotType::VendorSpecific(32768),
                crash_consistency: ConsistencyLevel::Crash,
                destination: destination.clone(),
                differential_backup_base: None,
                differential_backup_base_manifest_id: None,
                progress_sender: None,
            },
            "backup",
        )
        .await
        .expect("new backup should complete");

        assert_eq!(result.virtual_machine.name, virtual_machine_id);
        assert_eq!(result.destination, destination);
        assert!(!result.backup_id.to_string().is_empty());
        assert!(!result.reference_point.path.to_string().is_empty());

        reference_point_service.destroy(&result.reference_point).await.expect("backup_new reference point should be destroyed");
    }
}
