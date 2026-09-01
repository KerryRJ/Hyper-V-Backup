#![allow(non_snake_case)]

use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename = "Msvm_ComputerSystem")]
pub(crate) struct ComputerSystem {
    pub(crate) AvailableRequestedStates: Option<Vec<u16>>,
    pub(crate) Caption: Option<String>,    // "Virtual Machine" or "Hosting Computer System"
    pub(crate) CommunicationStatus: Option<u16>,
    pub(crate) CreationClassName: Option<String>,
    pub(crate) Dedicated: Option<Vec<u16>>,
    pub(crate) Description: Option<String>,    // "Microsoft Virtual Computer System" or "Micrososft Hosting Computer System"
    pub(crate) DetailedStatus: Option<u16>,
    pub(crate) ElementName: Option<String>,
    pub(crate) EnabledDefault: Option<u16>,   // 2
    pub(crate) EnabledState: Option<u16>, // 2
    pub(crate) EnhancedSessionModeState: Option<u16>,
    pub(crate) HealthState: Option<u16>,  // 5
    pub(crate) IdentifyingDescriptions: Option<Vec<String>>,
    pub(crate) InstanceID: Option<String>,
    pub(crate) InstallDate: Option<WMIDateTime>,
    pub(crate) LastSuccessfulBackupTime: Option<WMIDateTime>,
    pub(crate) Name: String,   // GUID
    pub(crate) NameFormat: Option<String>,
    pub(crate) NumberOfNumaNodes: Option<u16>,
    pub(crate) OnTimeInMilliseconds: Option<u64>,
    pub(crate) OperatingStatus: Option<u16>,
    pub(crate) OperationalStatus: Option<Vec<u16>>,
    pub(crate) OtherDedicatedDescriptions: Option<Vec<String>>,
    pub(crate) OtherEnabledState: Option<String>,
    pub(crate) OtherIdentifyingInfo: Option<Vec<String>>,
    pub(crate) PowerManagementCapabilities: Option<Vec<u16>>,
    pub(crate) PrimaryOwnerContact: Option<String>,
    pub(crate) PrimaryOwnerName: Option<String>,
    pub(crate) PrimaryStatus: Option<u16>,
    pub(crate) ProcessID: Option<u32>,
    pub(crate) ReplicationMode: Option<u16>,
    pub(crate) RequestedState: Option<u16>,
    pub(crate) ResetCapability: Option<u16>,  // 1
    pub(crate) Roles: Option<Vec<String>>,
    pub(crate) Status: Option<String>,
    pub(crate) StatusDescriptions: Option<Vec<String>>,
    pub(crate) TimeOfLastConfigurationChange: Option<WMIDateTime>,
    pub(crate) TimeOfLastStateChange: Option<WMIDateTime>,
    pub(crate) TransitioningToState: Option<u16>,
}