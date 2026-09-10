use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "Msvm_VirtualSystemReferencePointService")]
pub(crate) struct VirtualSystemReferencePointServiceOut {
    #[serde(rename = "__Path")]
    pub(super) path: String,
}
