use super::*;
use crate::{infra::VirtualSystemReferencePoint, model::*};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct BackupResult {
    pub backup_id: BackupId,
    pub virtual_machine: VirtualMachine,
    pub destination: PathBuf,
    pub reference_point: VirtualSystemReferencePoint,
    pub completed_at: DateTime<Utc>,
}
