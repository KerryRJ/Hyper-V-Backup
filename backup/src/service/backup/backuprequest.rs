use std::path::PathBuf;

use crate::infra::VirtualMachineId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupRequest {
    pub virtual_machine_id: VirtualMachineId,
    pub destination: PathBuf,
}
