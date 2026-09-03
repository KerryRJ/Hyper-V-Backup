use crate::model::{ConsistencyLevel, Error, ReferencePointType, VmId, WmiPath};
use crate::wmi::msvm;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePoint {
    path: WmiPath,
    instance_id: String,
    reference_point_type: ReferencePointType,
    consistency_level: ConsistencyLevel,
    virtual_system_identifier: VmId,
    has_associated_data: bool,
    virtual_disk_identifiers: Vec<String>,
    resilient_change_tracking_identifiers: Vec<String>,
}

impl ReferencePoint {
    pub fn path(&self) -> &WmiPath {
        &self.path
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    pub fn reference_point_type(&self) -> ReferencePointType {
        self.reference_point_type
    }

    pub fn consistency_level(&self) -> ConsistencyLevel {
        self.consistency_level
    }

    pub fn virtual_system_identifier(&self) -> VmId {
        self.virtual_system_identifier
    }

    pub fn has_associated_data(&self) -> bool {
        self.has_associated_data
    }

    pub fn virtual_disk_identifiers(&self) -> &[String] {
        &self.virtual_disk_identifiers
    }

    pub fn resilient_change_tracking_identifiers(&self) -> &[String] {
        &self.resilient_change_tracking_identifiers
    }
}

impl TryFrom<msvm::referencepoint::ReferencePoint> for ReferencePoint {
    type Error = Error;

    fn try_from(reference_point: msvm::referencepoint::ReferencePoint) -> Result<Self, Self::Error> {
        Ok(Self {
            path: WmiPath::from(reference_point.__Path),
            instance_id: reference_point.InstanceID,
            reference_point_type: ReferencePointType::try_from(reference_point.ReferencePointType).map_err(|_| Error::InvalidField("ReferencePointType"))?,
            consistency_level: ConsistencyLevel::try_from(reference_point.ConsistencyLevel).map_err(|_| Error::InvalidField("ConsistencyLevel"))?,
            virtual_system_identifier: VmId::parse_str(&reference_point.VirtualSystemIdentifier).map_err(|_| Error::InvalidField("VirtualSystemIdentifier"))?,
            has_associated_data: reference_point.HasAssociatedData,
            virtual_disk_identifiers: reference_point.VirtualDiskIdentifiers,
            resilient_change_tracking_identifiers: reference_point.ResilientChangeTrackingIdentifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_reference_point_properties() {
        let reference_point = msvm::referencepoint::ReferencePoint {
            __Path: "path".into(),
            InstanceID: "instance".into(),
            ReferencePointType: 1,
            ConsistencyLevel: 2,
            VirtualSystemIdentifier: "11111111-1111-1111-1111-111111111111".into(),
            HasAssociatedData: true,
            VirtualDiskIdentifiers: vec!["disk".into()],
            ResilientChangeTrackingIdentifiers: vec!["rct".into()],
        };

        let model = ReferencePoint::try_from(reference_point).unwrap();

        assert_eq!(model.path().as_str(), "path");
        assert_eq!(model.instance_id(), "instance");
        assert_eq!(model.reference_point_type(), ReferencePointType::RctBased);
        assert_eq!(model.consistency_level(), ConsistencyLevel::Crash);
        assert_eq!(model.virtual_system_identifier(), VmId::parse_str("11111111-1111-1111-1111-111111111111").unwrap());
        assert!(model.has_associated_data());
        assert_eq!(model.virtual_disk_identifiers(), ["disk".to_string()]);
        assert_eq!(model.resilient_change_tracking_identifiers(), ["rct".to_string()]);
    }
}