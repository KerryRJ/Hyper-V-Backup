#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "Msvm_VirtualSystemManagementService")]
pub(super) struct VirtualSystemManagementServiceClass {
    pub(super) __Path: String,
}
