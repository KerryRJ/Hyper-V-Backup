use std::path::PathBuf;

use crate::infra::*;
use crate::model::referencepointservice::ConsistencyLevel;
use crate::model::*;

#[derive(Clone, Debug)]
pub struct BackupRequest {
    pub virtual_machine: VirtualMachine,
    pub snapshot_type: SnapshotType,
    pub crash_consistency: ConsistencyLevel,
    pub destination: PathBuf,
    pub differential_backup_base: Option<VirtualSystemReferencePoint>,
}
