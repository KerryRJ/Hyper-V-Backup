use crate::model::{ReferencePointId, VirtualMachine};

use super::referencepointsettingdata::ReferencePointSettingData;
use crate::model::ReferencePointType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointCreateRequest {
    pub affected_system: VirtualMachine,
    pub reference_point_settings: Option<ReferencePointSettingData>,
    pub reference_point_type: ReferencePointType,
    pub resulting_reference_point: Option<ReferencePointId>,
}
