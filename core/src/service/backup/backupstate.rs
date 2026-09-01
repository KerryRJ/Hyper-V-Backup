use super::BackupFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackupState {
	Queued,
	Running,
	Completed,
	Cancelled,
	Failed(BackupFailure),
}
