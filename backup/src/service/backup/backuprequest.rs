use std::path::PathBuf;

use crate::model::VmId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupRequest {
    pub virtual_machine_id: VmId,
    pub destination: PathBuf,
}
