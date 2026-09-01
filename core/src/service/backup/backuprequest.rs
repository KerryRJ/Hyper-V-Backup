use std::path::PathBuf;

use crate::model::VmId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointRequest {
	pub affected_system: String,
	pub reference_point_settings: String,
	pub reference_point_type: u16,
	pub resulting_reference_point: String,
	pub retain_for_incremental: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupRequest {
	pub virtual_machine_id: VmId,
	pub destination: PathBuf,
}
