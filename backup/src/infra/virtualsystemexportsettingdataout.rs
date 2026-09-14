use super::{BackupIntent, CaptureLiveState, CopySnapshotConfiguration};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_VirtualSystemExportSettingData")]
pub(crate) struct VirtualSystemExportSettingDataOut {
    pub(super) __Path: String,
    pub(super) BackupIntent: BackupIntent,
    pub(super) Caption: String,
    pub(super) CaptureLiveState: CaptureLiveState,
    pub(super) CopySnapshotConfiguration: CopySnapshotConfiguration,
    pub(super) CopyVmRuntimeInformation: bool,
    pub(super) CopyVmStorage: bool,
    pub(super) CreateVmExportSubdirectory: bool,
    pub(super) Description: String,
    pub(super) DifferentialBackupBase: String,
    pub(super) DisableDifferentialOfIgnoredStorage: bool,
    pub(super) ElementName: String,
    pub(super) ExcludedVirtualHardDisks: Vec<String>,
    pub(super) ExportForLiveMigration: bool,
    pub(super) InstanceID: String,
    pub(super) SnapshotVirtualSystem: String,
}
