#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "Msvm_VirtualSystemReferencePointService")]
pub(super) struct VirtualSystemReferencePointServiceClass;
