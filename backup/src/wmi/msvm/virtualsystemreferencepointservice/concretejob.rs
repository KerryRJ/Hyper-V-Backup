#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct ConcreteJobConfiguration {
    pub(super) InstanceID: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ConcreteJob {
    pub(super) InstanceID: String,
    pub(super) JobState: u16,
    pub(super) ErrorCode: u16,
    pub(super) ErrorDescription: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "__InstanceModificationEvent")]
pub(super) struct ConcreteJobModificationEvent {
    pub(super) TargetInstance: ConcreteJob,
}
