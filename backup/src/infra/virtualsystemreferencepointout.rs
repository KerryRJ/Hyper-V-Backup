use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_VirtualSystemReferencePoint ")]
pub(crate) struct VirtualSystemReferencePointOut {
    pub(super) __Path: String,
    pub(super) ConsistencyLevel: u16,
    pub(super) HasAssociatedData: bool,
    pub(super) InstanceID: String,
    pub(super) ReferencePointType: u16,
    pub(super) ResilientChangeTrackingIdentifiers: Vec<String>,
    pub(super) VirtualDiskIdentifiers: Vec<String>,
    pub(super) VirtualSystemIdentifier: String,
}
