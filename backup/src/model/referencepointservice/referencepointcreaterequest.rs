use super::ReferencePointType;
use crate::model::{ReferencePointId, ReferencePointSettings, VirtualMachine};

#[derive(Clone, Debug)]
pub struct ReferencePointCreateRequest {
    pub affected_system: VirtualMachine,
    pub reference_point_settings: Option<ReferencePointSettings>,
    pub reference_point_type: ReferencePointType,
    pub resulting_reference_point: Option<ReferencePointId>,
}
