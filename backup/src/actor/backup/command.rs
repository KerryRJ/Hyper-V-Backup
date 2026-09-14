use crate::model::*;

pub(crate) enum Command {
    Start {
        id: BackupId,
        request: BackupRequest,
    },
    StartWithReferencePoint {
        id: BackupId,
        request: BackupRequest,
        // reference_point: ReferencePointCreateRequest,
        // options: ReferencePointBackupOptions,
    },
}
