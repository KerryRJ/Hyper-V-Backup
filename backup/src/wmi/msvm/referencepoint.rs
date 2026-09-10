#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename = "Msvm_VirtualSystemReferencePoint")]
pub(crate) struct ReferencePoint {
    pub(crate) __Path: String,
    pub(crate) InstanceID: String,
    pub(crate) ReferencePointType: u16,
    pub(crate) ConsistencyLevel: u16,
    pub(crate) VirtualSystemIdentifier: String,
    pub(crate) HasAssociatedData: bool,
    pub(crate) VirtualDiskIdentifiers: Vec<String>,
    pub(crate) ResilientChangeTrackingIdentifiers: Vec<String>, // Only valid for RCT reference points
}
