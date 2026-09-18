use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_ImageManagementService")]
pub(crate) struct ImageManagementServiceOut {
    pub(super) AvailableRequestedStates: Option<Vec<u16>>,
    pub(super) Caption: String,
    pub(super) CommunicationStatus: Option<u16>,
    pub(super) CreationClassName: String,
    pub(super) Description: String,
    pub(super) DetailedStatus: Option<u16>,
    pub(super) ElementName: String,
    pub(super) EnabledDefault: Option<u16>,
    pub(super) EnabledState: Option<u16>,
    pub(super) HealthState: Option<u16>,
    pub(super) InstanceID: Option<String>,
    pub(super) InstallDate: WMIDateTime,
    pub(super) Name: String,
    pub(super) OperationalStatus: Vec<u16>,
    pub(super) OperatingStatus: Option<u16>,
    pub(super) OtherEnabledState: Option<String>,
    #[serde(rename = "__Path")]
    pub(super) Path: String,
    pub(super) PrimaryOwnerContact: Option<String>,
    pub(super) PrimaryOwnerName: Option<String>,
    pub(super) PrimaryStatus: Option<u16>,
    pub(super) RequestedState: Option<u16>,
    pub(super) StartMode: Option<String>,
    pub(super) Started: bool,
    pub(super) Status: Option<String>,
    pub(super) StatusDescriptions: Vec<String>,
    pub(super) SystemCreationClassName: String,
    pub(super) SystemName: String,
    pub(super) TimeOfLastStateChange: WMIDateTime,
    pub(super) TransitioningToState: Option<u16>,
}
