use crate::infra::VirtualMachineId;
use crate::model::virtualmachine::{
    communicationstate, dedicated, detailedstatus, enableddefault, enabledstate, enhancedsessionmodestate, healthstate, operatingstatus, operationalstatus, powermanagementcapabilities, primarystatus, replicationmode, requestedstate,
    resetcapability, transitioningtostate,
};
use crate::model::{Error, WmiPath};
use crate::infra::ComputerSystemOut;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualMachine {
    available_requested_states: Vec<requestedstate::RequestedState>,
    caption: String,
    communication_status: Option<communicationstate::CommunicationStatus>,
    created: DateTime<Utc>,                          // From InstallDate
    creation_class_name: String,                     // Always set to "Msvm_ComputerSystem"
    dedicated: Vec<dedicated::Dedicated>,            // Always set to NotDedicated = 0
    description: String,                             // Will be "Microsoft Virtual Computer System" or "Microsoft Hosting Computer System"
    detailed_status: Option<detailedstatus::DetailedStatus>, // Complements PrimaryStatus
    enabled_default: enableddefault::EnabledDefault, // Default Enabled = 2 for a physical computer
    enabled_state: enabledstate::EnabledState,       // 2 is only for physical computer. There is no default for a VM
    enhanced_session_mode_state: enhancedsessionmodestate::EnhancedSessionModeState,
    health_state: healthstate::HealthState, // Default Ok = 5
    id: VirtualMachineId,                                       // From Name
    identifying_descriptions: Vec<String>,  // Always set to null
    instance_id: Option<String>,
    last_successful_backup_time: Option<DateTime<Utc>>,
    name: String,                                       // From ElementName
    name_format: Option<String>,                                // Always set to null
    number_of_numa_nodes: u16,                          // Set to null for the management OS
    on_time: Duration,                                  // From OnTimeInMilliseconds
    operating_status: Option<operatingstatus::OperatingStatus>, // Null means not implemented
    operational_status: operationalstatus::OperationalStatus,
    other_dedicated_descriptions: Vec<String>,                                                    // Always set to null
    other_enabled_state: Option<String>,                                                                  // Must be null when EnabledState is not Other. Always set to null
    other_identifying_info: Vec<String>,                                                          // Always set to null
    path: WmiPath,
    power_management_capabilities: Vec<powermanagementcapabilities::PowerManagementCapabilities>, // Not used
    primary_owner_contact: Option<String>,                                                                // Always set to null
    primary_owner_name: Option<String>,                                                                   // Always set to null
    primary_status: Option<primarystatus::PrimaryStatus>,                                                 // Used in conjunction with DetailedStatus. Null indicates not implemented.
    process_id: Option<u32>,
    replication_mode: replicationmode::ReplicationMode,
    requested_state: requestedstate::RequestedState,
    reset_capability: resetcapability::ResetCapability, // Always set to Other = 1
    roles: Vec<String>,                                 // Always set to null
    status: String,                                     // Not used
    status_descriptions: Vec<String>,
    time_of_last_configuration_change: DateTime<Utc>,
    time_of_last_state_change: DateTime<Utc>,
    transitioning_to_state: Option<transitioningtostate::TransitioningToState>, // Not used
}

impl std::fmt::Display for VirtualMachine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VirtualMachine")
            .field("available_requested_states", &self.available_requested_states)
            .field("caption", &self.caption)
            .field("communication_status", &self.communication_status)
            .field("created", &self.created)
            .field("creation_class_name", &self.creation_class_name)
            .field("dedicated", &self.dedicated)
            .field("description", &self.description)
            .field("detailed_status", &self.detailed_status)
            .field("enabled_default", &self.enabled_default)
            .field("enabled_state", &self.enabled_state)
            .field("enhanced_session_mode_state", &self.enhanced_session_mode_state)
            .field("health_state", &self.health_state)
            .field("id", &self.id)
            .field("identifying_descriptions", &self.identifying_descriptions)
            .field("instance_id", &self.instance_id)
            .field("last_successful_backup_time", &self.last_successful_backup_time)
            .field("name", &self.name)
            .field("name_format", &self.name_format)
            .field("number_of_numa_nodes", &self.number_of_numa_nodes)
            .field("on_time", &self.on_time)
            .field("operating_status", &self.operating_status)
            .field("operational_status", &self.operational_status)
            .field("other_dedicated_descriptions", &self.other_dedicated_descriptions)
            .field("other_enabled_state", &self.other_enabled_state)
            .field("other_identifying_info", &self.other_identifying_info)
            .field("path", &self.path)
            .field("power_management_capabilities", &self.power_management_capabilities)
            .field("primary_owner_contact", &self.primary_owner_contact)
            .field("primary_owner_name", &self.primary_owner_name)
            .field("primary_status", &self.primary_status)
            .field("process_id", &self.process_id)
            .field("replication_mode", &self.replication_mode)
            .field("requested_state", &self.requested_state)
            .field("reset_capability", &self.reset_capability)
            .field("roles", &self.roles)
            .field("status", &self.status)
            .field("status_descriptions", &self.status_descriptions)
            .field("time_of_last_configuration_change", &self.time_of_last_configuration_change)
            .field("time_of_last_state_change", &self.time_of_last_state_change)
            .field("transitioning_to_state", &self.transitioning_to_state)
            .finish()
    }
}

impl VirtualMachine {
    pub fn path(&self) -> &WmiPath {
        &self.path
    }

    pub fn available_requested_states(&self) -> Option<&[requestedstate::RequestedState]> {
        Some(&self.available_requested_states)
    }
    pub fn caption(&self) -> Option<&str> {
        Some(&self.caption)
    }
    pub fn communication_status(&self) -> Option<&communicationstate::CommunicationStatus> {
        self.communication_status.as_ref()
    }
    pub fn created(&self) -> Option<DateTime<Utc>> {
        Some(self.created)
    }
    pub fn creation_class_name(&self) -> Option<&str> {
        Some(&self.creation_class_name)
    }
    pub fn dedicated(&self) -> Option<&[dedicated::Dedicated]> {
        Some(&self.dedicated)
    }
    pub fn description(&self) -> Option<&str> {
        Some(&self.description)
    }
    pub fn detailed_status(&self) -> Option<&detailedstatus::DetailedStatus> {
        self.detailed_status.as_ref()
    }
    pub fn enabled_default(&self) -> Option<&enableddefault::EnabledDefault> {
        Some(&self.enabled_default)
    }
    pub fn enabled_state(&self) -> Option<&enabledstate::EnabledState> {
        Some(&self.enabled_state)
    }
    pub fn enhanced_session_mode_state(&self) -> Option<&enhancedsessionmodestate::EnhancedSessionModeState> {
        Some(&self.enhanced_session_mode_state)
    }
    pub fn health_state(&self) -> Option<&healthstate::HealthState> {
        Some(&self.health_state)
    }
    pub fn id(&self) -> VirtualMachineId {
        self.id
    }
    pub fn identifying_descriptions(&self) -> Option<&[String]> {
        Some(&self.identifying_descriptions)
    }
    pub fn instance_id(&self) -> Option<&str> {
        self.instance_id.as_deref()
    }
    pub fn last_successful_backup_time(&self) -> Option<DateTime<Utc>> {
        self.last_successful_backup_time
    }
    pub fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
    pub fn name_format(&self) -> Option<&str> {
        self.name_format.as_deref()
    }
    pub fn number_of_numa_nodes(&self) -> Option<u16> {
        Some(self.number_of_numa_nodes)
    }
    pub fn on_time(&self) -> Option<Duration> {
        Some(self.on_time)
    }
    pub fn operating_status(&self) -> Option<&operatingstatus::OperatingStatus> {
        self.operating_status.as_ref()
    }
    pub fn operational_status(&self) -> Option<&operationalstatus::OperationalStatus> {
        Some(&self.operational_status)
    }
    pub fn other_dedicated_descriptions(&self) -> Option<&[String]> {
        Some(&self.other_dedicated_descriptions)
    }
    pub fn other_enabled_state(&self) -> Option<&str> {
        self.other_enabled_state.as_deref()
    }
    pub fn other_identifying_info(&self) -> Option<&[String]> {
        Some(&self.other_identifying_info)
    }
    pub fn power_management_capabilities(&self) -> Option<&[powermanagementcapabilities::PowerManagementCapabilities]> {
        Some(&self.power_management_capabilities)
    }
    pub fn primary_owner_contact(&self) -> Option<&str> {
        self.primary_owner_contact.as_deref()
    }
    pub fn primary_owner_name(&self) -> Option<&str> {
        self.primary_owner_name.as_deref()
    }
    pub fn primary_status(&self) -> Option<&primarystatus::PrimaryStatus> {
        self.primary_status.as_ref()
    }
    pub fn process_id(&self) -> Option<u32> {
        self.process_id
    }
    pub fn replication_mode(&self) -> Option<&replicationmode::ReplicationMode> {
        Some(&self.replication_mode)
    }
    pub fn requested_state(&self) -> Option<&requestedstate::RequestedState> {
        Some(&self.requested_state)
    }
    pub fn reset_capability(&self) -> Option<&resetcapability::ResetCapability> {
        Some(&self.reset_capability)
    }
    pub fn roles(&self) -> Option<&[String]> {
        Some(&self.roles)
    }
    pub fn status(&self) -> Option<&str> {
        Some(&self.status)
    }
    pub fn status_descriptions(&self) -> Option<&[String]> {
        Some(&self.status_descriptions)
    }
    pub fn time_of_last_configuration_change(&self) -> Option<DateTime<Utc>> {
        Some(self.time_of_last_configuration_change)
    }
    pub fn time_of_last_state_change(&self) -> Option<DateTime<Utc>> {
        Some(self.time_of_last_state_change)
    }
    pub fn transitioning_to_state(&self) -> Option<&transitioningtostate::TransitioningToState> {
        self.transitioning_to_state.as_ref()
    }
}

impl TryFrom<ComputerSystemOut> for VirtualMachine {
    type Error = Error;

    fn try_from(computer_system: ComputerSystemOut) -> Result<Self, Self::Error> {
        Ok(Self {
            path: WmiPath::from(computer_system.path),
            available_requested_states: computer_system
                .AvailableRequestedStates
                .into_iter()
                .map(requestedstate::RequestedState::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Error::InvalidField("AvailableRequestedStates"))?,
            caption: computer_system.Caption,
            communication_status: computer_system
                .CommunicationStatus
                .map(|value| communicationstate::CommunicationStatus::try_from(value).map_err(|_| Error::InvalidField("CommunicationStatus")))
                .transpose()?,
            created: computer_system.InstallDate.0.with_timezone(&Utc),
            creation_class_name: computer_system.CreationClassName, // Always set to "Msvm_ComputerSystem"
            dedicated: computer_system
                .Dedicated
                .into_iter()
                .map(dedicated::Dedicated::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Error::InvalidField("Dedicated"))?, // Always set to NotDedicated = 0
            description: computer_system.Description,               // Will be "Microsoft Virtual Computer System" or "Microsoft Hosting Computer System"
            detailed_status: computer_system
                .DetailedStatus
                .map(|value| detailedstatus::DetailedStatus::try_from(value).map_err(|_| Error::InvalidField("DetailedStatus")))
                .transpose()?, // Complements PrimaryStatus
            enabled_default: enableddefault::EnabledDefault::try_from(computer_system.EnabledDefault).map_err(|_| Error::InvalidField("EnabledDefault"))?,
            enabled_state: enabledstate::EnabledState::try_from(computer_system.EnabledState).map_err(|_| Error::InvalidField("EnabledState"))?,
            enhanced_session_mode_state: enhancedsessionmodestate::EnhancedSessionModeState::try_from(computer_system.EnhancedSessionModeState).map_err(|_| Error::InvalidField("EnhancedSessionModeState"))?,
            health_state: healthstate::HealthState::try_from(computer_system.HealthState).map_err(|_| Error::InvalidField("HealthState"))?, // Default Ok = 5
            id: VirtualMachineId::new(Uuid::parse_str(&computer_system.Name)?).ok_or(Error::InvalidField("Name"))?,
            identifying_descriptions: computer_system.IdentifyingDescriptions, // Always set to null
            instance_id: computer_system.InstanceID,
            last_successful_backup_time: computer_system.LastSuccessfulBackupTime.map(|v| v.0.with_timezone(&Utc)),
            name: computer_system.ElementName,
            name_format: computer_system.NameFormat,                 // Always set to null
            number_of_numa_nodes: computer_system.NumberOfNumaNodes, // Set to null for the management OS
            on_time: Duration::from_millis(computer_system.OnTimeInMilliseconds),
            operating_status: computer_system
                .OperatingStatus
                .map(|value| operatingstatus::OperatingStatus::try_from(value).map_err(|_| Error::InvalidField("OperatingStatus")))
                .transpose()?, // Null means not implemented
            operational_status: operationalstatus::OperationalStatus::from_values(computer_system.OperationalStatus).map_err(|_| Error::InvalidField("OperationalStatus"))?,
            other_dedicated_descriptions: computer_system.OtherDedicatedDescriptions, // Always set to null
            other_enabled_state: computer_system.OtherEnabledState,                   // Must be null when EnabledState is not Other. Always set to null
            other_identifying_info: computer_system.OtherIdentifyingInfo,             // Always set to null
            power_management_capabilities: computer_system
                .PowerManagementCapabilities
                .into_iter()
                .map(powermanagementcapabilities::PowerManagementCapabilities::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Error::InvalidField("PowerManagementCapabilities"))?, // Not used
            primary_owner_contact: computer_system.PrimaryOwnerContact,               // Always set to null
            primary_owner_name: computer_system.PrimaryOwnerName,                     // Always set to null
            primary_status: computer_system
                .PrimaryStatus
                .map(|value| primarystatus::PrimaryStatus::try_from(value).map_err(|_| Error::InvalidField("PrimaryStatus")))
                .transpose()?, // Used in conjunction with DetailedStatus. Null indicates not implemented.
            process_id: computer_system.ProcessID,
            replication_mode: replicationmode::ReplicationMode::try_from(computer_system.ReplicationMode).map_err(|_| Error::InvalidField("ReplicationMode"))?,
            requested_state: requestedstate::RequestedState::try_from(computer_system.RequestedState).map_err(|_| Error::InvalidField("RequestedState"))?,
            reset_capability: resetcapability::ResetCapability::try_from(computer_system.ResetCapability).map_err(|_| Error::InvalidField("ResetCapability"))?, // Always set to Other = 1
            roles: computer_system.Roles,   // Always set to null
            status: computer_system.Status, // Not used
            status_descriptions: computer_system.StatusDescriptions,
            time_of_last_configuration_change: computer_system.TimeOfLastConfigurationChange.0.with_timezone(&Utc),
            time_of_last_state_change: computer_system.TimeOfLastStateChange.0.with_timezone(&Utc),
            transitioning_to_state: computer_system
                .TransitioningToState
                .map(|value| transitioningtostate::TransitioningToState::try_from(value).map_err(|_| Error::InvalidField("TransitioningToState")))
                .transpose()?, // Not used
        })
    }
}
