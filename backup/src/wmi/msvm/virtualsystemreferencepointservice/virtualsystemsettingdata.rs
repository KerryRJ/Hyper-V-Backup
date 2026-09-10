#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "Msvm_VirtualSystemSettingData")]
pub(super) struct VirtualSystemSettingData {
    InstanceID: String,
    IncrementalBackupEnabled: bool,
    VirtualSystemType: Option<String>,
}

#[derive(Serialize)]
pub(super) struct ModifySystemSettingsInput {
    SystemSettings: String,
}

impl VirtualSystemSettingData {
    pub(super) fn is_current(&self) -> bool {
        self.VirtualSystemType.as_deref() == Some("Microsoft:Hyper-V:System:Realized")
    }

    pub(super) fn incremental_backup_enabled(&self) -> bool {
        self.IncrementalBackupEnabled
    }

    pub(super) fn enable_incremental_backup_input(&self) -> ModifySystemSettingsInput {
        ModifySystemSettingsInput {
            SystemSettings: format!(
                "<INSTANCE CLASSNAME=\"Msvm_VirtualSystemSettingData\"><PROPERTY NAME=\"InstanceID\" TYPE=\"string\"><VALUE>{}</VALUE></PROPERTY><PROPERTY NAME=\"IncrementalBackupEnabled\" TYPE=\"boolean\"><VALUE>TRUE</VALUE></PROPERTY></INSTANCE>",
                self.InstanceID,
            ),
        }
    }
}
