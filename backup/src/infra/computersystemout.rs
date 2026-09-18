use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename = "Msvm_ComputerSystem")]
pub(crate) struct ComputerSystemOut {
    pub(crate) AvailableRequestedStates: Vec<u16>,
    pub(crate) Caption: String, // "Virtual Machine" or "Hosting Computer System"
    pub(crate) CommunicationStatus: Option<u16>,
    pub(crate) CreationClassName: String,
    pub(crate) Dedicated: Vec<u16>,
    pub(crate) Description: String, // "Microsoft Virtual Computer System" or "Micrososft Hosting Computer System"
    pub(crate) DetailedStatus: Option<u16>,
    pub(crate) ElementName: String,
    pub(crate) EnabledDefault: u16, // 2
    pub(crate) EnabledState: u16,   // 2
    pub(crate) EnhancedSessionModeState: u16,
    pub(crate) HealthState: u16, // 5
    pub(crate) IdentifyingDescriptions: Vec<String>,
    pub(crate) InstanceID: Option<String>,
    pub(crate) InstallDate: WMIDateTime,
    pub(crate) LastSuccessfulBackupTime: Option<WMIDateTime>,
    pub(crate) Name: String, // GUID
    pub(crate) NameFormat: Option<String>,
    pub(crate) NumberOfNumaNodes: u16,
    pub(crate) OnTimeInMilliseconds: u64,
    pub(crate) OperatingStatus: Option<u16>,
    pub(crate) OperationalStatus: Vec<u16>,
    pub(crate) OtherDedicatedDescriptions: Vec<String>,
    pub(crate) OtherEnabledState: Option<String>,
    pub(crate) OtherIdentifyingInfo: Vec<String>,
    #[serde(rename = "__Path")]
    pub(crate) path: String,
    pub(crate) PowerManagementCapabilities: Vec<u16>,
    pub(crate) PrimaryOwnerContact: Option<String>,
    pub(crate) PrimaryOwnerName: Option<String>,
    pub(crate) PrimaryStatus: Option<u16>,
    pub(crate) ProcessID: Option<u32>,
    pub(crate) ReplicationMode: u16,
    pub(crate) RequestedState: u16,
    pub(crate) ResetCapability: u16, // 1
    pub(crate) Roles: Vec<String>,
    pub(crate) Status: String,
    pub(crate) StatusDescriptions: Vec<String>,
    pub(crate) TimeOfLastConfigurationChange: WMIDateTime,
    pub(crate) TimeOfLastStateChange: WMIDateTime,
    pub(crate) TransitioningToState: Option<u16>,
}

impl std::fmt::Display for ComputerSystemOut {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ComputerSystemOut")
            .field("AvailableRequestedStates", &self.AvailableRequestedStates)
            .field("Caption", &self.Caption)
            .field("CommunicationStatus", &self.CommunicationStatus)
            .field("CreationClassName", &self.CreationClassName)
            .field("Dedicated", &self.Dedicated)
            .field("Description", &self.Description)
            .field("DetailedStatus", &self.DetailedStatus)
            .field("ElementName", &self.ElementName)
            .field("EnabledDefault", &self.EnabledDefault)
            .field("EnabledState", &self.EnabledState)
            .field("EnhancedSessionModeState", &self.EnhancedSessionModeState)
            .field("HealthState", &self.HealthState)
            .field("IdentifyingDescriptions", &self.IdentifyingDescriptions)
            .field("InstanceID", &self.InstanceID)
            .field("InstallDate", &self.InstallDate)
            .field("LastSuccessfulBackupTime", &self.LastSuccessfulBackupTime)
            .field("Name", &self.Name)
            .field("NameFormat", &self.NameFormat)
            .field("NumberOfNumaNodes", &self.NumberOfNumaNodes)
            .field("OnTimeInMilliseconds", &self.OnTimeInMilliseconds)
            .field("OperatingStatus", &self.OperatingStatus)
            .field("OperationalStatus", &self.OperationalStatus)
            .field("OtherDedicatedDescriptions", &self.OtherDedicatedDescriptions)
            .field("OtherEnabledState", &self.OtherEnabledState)
            .field("OtherIdentifyingInfo", &self.OtherIdentifyingInfo)
            .field("path", &self.path)
            .field("PowerManagementCapabilities", &self.PowerManagementCapabilities)
            .field("PrimaryOwnerContact", &self.PrimaryOwnerContact)
            .field("PrimaryOwnerName", &self.PrimaryOwnerName)
            .field("PrimaryStatus", &self.PrimaryStatus)
            .field("ProcessID", &self.ProcessID)
            .field("ReplicationMode", &self.ReplicationMode)
            .field("RequestedState", &self.RequestedState)
            .field("ResetCapability", &self.ResetCapability)
            .field("Roles", &self.Roles)
            .field("Status", &self.Status)
            .field("StatusDescriptions", &self.StatusDescriptions)
            .field("TimeOfLastConfigurationChange", &self.TimeOfLastConfigurationChange)
            .field("TimeOfLastStateChange", &self.TimeOfLastStateChange)
            .field("TransitioningToState", &self.TransitioningToState)
            .finish()
    }
}

impl Default for ComputerSystemOut {
    fn default() -> Self {
        let default_time = WMIDateTime(DateTime::UNIX_EPOCH.with_timezone(&FixedOffset::east_opt(0).unwrap()));
        Self {
            AvailableRequestedStates: Vec::new(),
            Caption: String::new(),
            CommunicationStatus: None,
            CreationClassName: String::new(),
            Dedicated: Vec::new(),
            Description: String::new(),
            DetailedStatus: None,
            ElementName: String::new(),
            EnabledDefault: 2,
            EnabledState: 2,
            EnhancedSessionModeState: 2,
            HealthState: 5,
            IdentifyingDescriptions: Vec::new(),
            InstanceID: None,
            InstallDate: default_time.clone(),
            LastSuccessfulBackupTime: None,
            Name: String::new(),
            NameFormat: None,
            NumberOfNumaNodes: 0,
            OnTimeInMilliseconds: 0,
            OperatingStatus: None,
            OperationalStatus: vec![2],
            OtherDedicatedDescriptions: Vec::new(),
            OtherEnabledState: None,
            OtherIdentifyingInfo: Vec::new(),
            path: String::new(),
            PowerManagementCapabilities: Vec::new(),
            PrimaryOwnerContact: None,
            PrimaryOwnerName: None,
            PrimaryStatus: None,
            ProcessID: None,
            ReplicationMode: 0,
            RequestedState: 12,
            ResetCapability: 1,
            Roles: Vec::new(),
            Status: String::new(),
            StatusDescriptions: Vec::new(),
            TimeOfLastConfigurationChange: default_time.clone(),
            TimeOfLastStateChange: default_time,
            TransitioningToState: None,
        }
    }
}
