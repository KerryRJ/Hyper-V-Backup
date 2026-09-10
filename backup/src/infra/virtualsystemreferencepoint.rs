use std::fmt;

use serde::{Deserialize, Deserializer};
use uuid::Uuid;

#[derive(Clone)]
pub(super) struct VirtualSystemReferencePoint {
    pub consistency_level: super::ConsistencyLevel, // rw
    pub has_associated_data: bool,                  // rw
    pub instance_id: super::InstanceId,             // r
    pub path: super::Path,
    pub reference_point_type: super::ReferencePointType,                              // rw
    pub resilient_change_tracking_identifiers: Vec<super::ResilientChangeTrackingId>, // r
    pub virtual_disk_identifiers: Vec<super::VirtualDiskId>,                          // r
    pub virtual_system_identifier: super::VirtualMachineId,                           // r
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
        let output = super::VirtualSystemReferencePointOut::deserialize(deserializer)?;
        let virtual_system_identifier = Uuid::parse_str(&output.VirtualSystemIdentifier).map_err(serde::de::Error::custom)?;
        let virtual_disk_identifiers = output
            .VirtualDiskIdentifiers
            .into_iter()
            .map(|value| Uuid::parse_str(&value).map(super::VirtualDiskId::from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;
        let resilient_change_tracking_identifiers = output
            .ResilientChangeTrackingIdentifiers
            .into_iter()
            .map(|value| Uuid::parse_str(&value).map(super::ResilientChangeTrackingId::from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            consistency_level: output.ConsistencyLevel.try_into().map_err(serde::de::Error::custom)?,
            has_associated_data: output.HasAssociatedData,
            instance_id: output.InstanceID.into(),
            path: output.__Path.into(),
            reference_point_type: output.ReferencePointType.try_into().map_err(serde::de::Error::custom)?,
            resilient_change_tracking_identifiers,
            virtual_disk_identifiers,
            virtual_system_identifier: virtual_system_identifier.into(),
        })
    }
}
