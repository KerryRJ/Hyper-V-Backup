use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_VirtualSystemSnapshotService")]
pub(crate) struct VirtualSystemSnapshotServiceOut {
    pub(super) AvailableRequestedStates: Option<Vec<u16>>, // r
    pub(super) Caption: String,                            // r
    pub(super) CommunicationStatus: Option<u16>,           // r
    pub(super) CreationClassName: String,                  // r
    pub(super) Description: String,                        // r
    pub(super) DetailedStatus: Option<u16>,                // r
    pub(super) ElementName: String,                        // r
    pub(super) EnabledDefault: Option<u16>,                // r    // Always 2 = Enabled
    pub(super) EnabledState: Option<u16>,                  // r    // Always 2 = Enabled
    pub(super) HealthState: Option<u16>,                   // r   // Always set to 5 = Ok but is a range 0 to 30 where 30 is completely non-functional
    pub(super) InstanceID: Option<String>,                 // r
    pub(super) InstallDate: WMIDateTime,                   // r
    pub(super) Name: String,                               // r
    pub(super) OperatingStatus: Option<u16>,               // r
    pub(super) OperationalStatus: Vec<u16>,                // r    // Always set to 2 = OK
    pub(super) OtherEnabledState: Option<String>,          // r
    #[serde(rename = "__Path")]
    pub(super) Path: String,
    pub(super) PrimaryOwnerContact: Option<String>, // r
    pub(super) PrimaryOwnerName: Option<String>,    // r
    pub(super) PrimaryStatus: Option<u16>,          // r
    pub(super) RequestedState: Option<u16>,         // r    // Always set to 12 = Not Applicable
    pub(super) StartMode: Option<String>,           // r
    pub(super) Started: bool,                       // r
    pub(super) Status: Option<String>,              // r
    pub(super) StatusDescriptions: Vec<String>,     // r
    pub(super) SystemCreationClassName: String,     // r
    pub(super) SystemName: String,                  // r
    pub(super) TimeOfLastStateChange: WMIDateTime,  // r
    pub(super) TransitioningToState: Option<u16>,   // r
}
