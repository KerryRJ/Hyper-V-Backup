use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Result {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
    #[serde(rename = "ResultingReferencePoint")]
    pub(super) resulting_reference_point: Option<String>,
    #[serde(rename = "ResultingSnapshot")]
    pub(super) resulting_snapshot: Option<String>,
}
