use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "__InstanceModificationEvent")]
#[allow(non_snake_case)]
pub(super) struct ConcreteJobModificationEvent {
    pub(super) TargetInstance: super::JobOut,
}