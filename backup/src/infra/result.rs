use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Result {
    #[serde(rename = "ReturnValue")]
    pub(super) return_value: u32,
    #[serde(rename = "Job")]
    pub(super) job: Option<String>,
}
