use crate::service::backup::{BackupId, BackupRequest, ReferencePointRequest};

pub(crate) enum Command {
	Start {
		id: BackupId,
		request: BackupRequest,
	},
	StartWithReferencePoint {
		id: BackupId,
		request: BackupRequest,
		reference_point: ReferencePointRequest,
	},
}
