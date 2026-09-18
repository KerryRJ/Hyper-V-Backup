use std::fmt;

use super::*;
use serde::{Deserialize, Deserializer};

#[derive(Clone)]
pub(crate) struct VirtualSystemReferencePoint {
    pub consistency_level: ConsistencyLevel, // rw
    pub has_associated_data: bool,           // rw
    pub instance_id: InstanceId,             // r
    pub path: Path,
    pub reference_point_type: crate::model::referencepointservice::ReferencePointType, // rw
    pub resilient_change_tracking_identifiers: Vec<ResilientChangeTrackingId>,         // r
    pub virtual_disk_identifiers: Vec<VirtualDiskId>,                                  // r
    pub virtual_system_identifier: VirtualMachineId,                                   // r
}

impl fmt::Debug for VirtualSystemReferencePoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VirtualSystemReferencePoint")
            .field("consistency_level", &self.consistency_level)
            .field("has_associated_data", &self.has_associated_data)
            .field("instance_id", &self.instance_id)
            .field("path", &self.path)
            .field("reference_point_type", &self.reference_point_type)
            .field("resilient_change_tracking_identifiers", &self.resilient_change_tracking_identifiers)
            .field("virtual_disk_identifiers", &self.virtual_disk_identifiers)
            .field("virtual_system_identifier", &self.virtual_system_identifier)
            .finish()
    }
}

impl<'de> Deserialize<'de> for VirtualSystemReferencePoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let output = VirtualSystemReferencePointOut::deserialize(deserializer)?;
        let virtual_system_identifier = VirtualMachineId::parse_str(&output.VirtualSystemIdentifier).map_err(serde::de::Error::custom)?;
        let virtual_disk_identifiers = output.VirtualDiskIdentifiers.into_iter().map(VirtualDiskId::from).collect::<Vec<_>>();
        let resilient_change_tracking_identifiers = output.ResilientChangeTrackingIdentifiers.into_iter().map(ResilientChangeTrackingId::from).collect::<Vec<_>>();
        let reference_point = Self {
            consistency_level: output.ConsistencyLevel.try_into().map_err(serde::de::Error::custom)?,
            has_associated_data: output.HasAssociatedData,
            instance_id: output.InstanceID.into(),
            path: output.__Path.into(),
            reference_point_type: output.ReferencePointType.try_into().map_err(serde::de::Error::custom)?,
            resilient_change_tracking_identifiers,
            virtual_disk_identifiers,
            virtual_system_identifier: virtual_system_identifier.into(),
        };
        Ok(reference_point)
    }
}

impl VirtualSystemReferencePoint {
    pub(crate) fn reference_point_id(&self) -> Result<ReferencePointId, crate::model::Error> {
        let reference_point_id = ReferencePointId::parse_str(self.instance_id.as_ref()).map_err(|error| match error {
            crate::model::ReferencePointIdError::InvalidUuid(error) => crate::model::Error::Uuid(error),
            crate::model::ReferencePointIdError::Nil => crate::model::Error::InvalidField("reference point identifier cannot be nil"),
        })?;
        Ok(reference_point_id)
    }
}
