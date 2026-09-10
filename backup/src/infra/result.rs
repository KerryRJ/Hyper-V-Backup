use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct ConvertToReferencePointResult {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateSnapshotResult {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct DestroySnapshotResult {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct DestroyReferencePointResult {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
}
