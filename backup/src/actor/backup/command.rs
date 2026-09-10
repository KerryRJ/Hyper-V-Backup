use crate::service::backup::{BackupId, BackupRequest, ReferencePointBackupOptions};
use crate::service::referencepointservice::ReferencePointCreateRequest;

pub(crate) enum Command {
    Start {
        id: BackupId,
        request: BackupRequest,
    },
    StartWithReferencePoint {
        id: BackupId,
        request: BackupRequest,
        reference_point: ReferencePointCreateRequest,
        options: ReferencePointBackupOptions,
    },
}
