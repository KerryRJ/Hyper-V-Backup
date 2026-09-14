use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum CopySnapshotConfiguration {
    ExportAllSnapshots,
    ExportNoSnapshots,
    ExportOneSnapshot,
    ExportOneSnapshotForBackup,
}

impl TryFrom<u8> for CopySnapshotConfiguration {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ExportAllSnapshots),
            1 => Ok(Self::ExportNoSnapshots),
            2 => Ok(Self::ExportOneSnapshot),
            3 => Ok(Self::ExportOneSnapshotForBackup),
            _ => Err("Copy snapshot configuration must be ExportAllSnapshots (0), ExportNoSnapshots (1), ExportOneSnapshot (2), or ExportOneSnapshotForBackup (3)"),
        }
    }
}

impl<'de> Deserialize<'de> for CopySnapshotConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
