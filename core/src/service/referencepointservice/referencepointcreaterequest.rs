use crate::model::{ReferencePointId, VmId};

use super::referencepointsettingsdata::ReferencePointSettingsData;
use super::referencepointtype::ReferencePointType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointCreateRequest {
    pub affected_system: VmId,
    pub reference_point_settings: Option<ReferencePointSettingsData>,
    pub reference_point_type: ReferencePointType,
    pub resulting_reference_point: Option<ReferencePointId>,
}
