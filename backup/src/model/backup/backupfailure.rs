#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackupFailure {
    InvalidRequest,
    VirtualMachineNotFound,
    Io,
    Wmi,
    BackendUnavailable,
    Internal,
}
