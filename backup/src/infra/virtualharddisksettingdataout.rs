use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_VirtualHardDiskSettingData")]
pub(crate) struct VirtualHardDiskSettingDataOut {
    pub(super) BlockSize: u32,
    pub(super) Caption: String,
    pub(super) DataAlignment: Option<u64>,
    pub(super) Description: String,
    pub(super) ElementName: String,
    pub(super) Format: u16,
    pub(super) InstanceID: String,
    pub(super) IsPmemCompatible: bool,
    pub(super) LogicalSectorSize: u32,
    pub(super) MaxInternalSize: u64,
    pub(super) ParentIdentifier: Option<String>,
    pub(super) ParentPath: String,
    pub(super) ParentTimestamp: Option<WMIDateTime>,
    pub(super) Path: String,
    pub(super) PhysicalSectorSize: u32,
    pub(super) PmemAddressAbstractionType: u16,
    pub(super) Type: u16,
    pub(super) VirtualDiskId: String,
}
