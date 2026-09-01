use super::BackupState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupStatus {
    pub state: BackupState,
    pub progress: u8,
}
