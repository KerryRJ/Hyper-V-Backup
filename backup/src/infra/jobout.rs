use serde::Deserialize;
use wmi::{WMIDateTime, WMIDuration};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(super) struct JobOut {
    pub(super) InstanceID: String,
    pub(super) Name: String,
    pub(crate) JobState: u16,
    pub(super) TimeOfLastStateChange: WMIDateTime,
    pub(super) TimeBeforeRemoval: WMIDuration,
}

impl JobOut {
    pub(super) fn instance_id(&self) -> &str {
        &self.InstanceID
    }
}
