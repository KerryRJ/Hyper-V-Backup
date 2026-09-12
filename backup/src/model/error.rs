use thiserror::Error;

use crate::model::VirtualMachineId;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid field: {0}")]
    InvalidField(&'static str),
    #[error("Invalid virtual machine uuid: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("Virtual machine not found: {0}")]
    VirtualMachineNotFound(VirtualMachineId),
    #[error("Invalid backup request: {0}")]
    InvalidBackupRequest(&'static str),
    #[error("Invalid backup schedule: {0}")]
    InvalidBackupSchedule(&'static str),
    #[error("Backup not found: {0}")]
    BackupNotFound(uuid::Uuid),
    #[error("Backup operation was cancelled")]
    BackupCancelled,
    #[error("Backup backend is not implemented")]
    BackupBackendUnavailable,
    #[error("Backup I/O operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Hyper-V WMI operation failed: {0}")]
    Wmi(#[from] wmi::WMIError),
}
