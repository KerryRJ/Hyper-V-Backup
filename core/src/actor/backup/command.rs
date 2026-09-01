use crate::service::backup::{BackupId, BackupRequest};

pub(crate) enum Command {
	Start {
		id: BackupId,
		request: BackupRequest,
	},
}
