use crate::infra::{ReferencePointId, ReferencePointType, VirtualSystemReferencePointSettingDataIn};
use crate::model::VirtualMachine;

#[derive(Clone, Debug)]
pub struct ReferencePointCreateRequest {
    pub affected_system: VirtualMachine,
    pub reference_point_settings: Option<VirtualSystemReferencePointSettingDataIn>,
    pub reference_point_type: ReferencePointType,
    pub resulting_reference_point: Option<ReferencePointId>,
}
