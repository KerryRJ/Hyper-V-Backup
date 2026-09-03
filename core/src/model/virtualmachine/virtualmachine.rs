use super::vmid::VmId;
use crate::model::Error;
use crate::model::virtualmachine::{
    communicationstate, dedicated, detailedstatus, enableddefault, enabledstate, enhancedsessionmodestate, healthstate, operatingstatus, operationalstatus, powermanagementcapabilities, primarystatus, replicationmode, requestedstate,
    resetcapability, transitioningtostate,
};
use crate::wmi::msvm::computersystem;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualMachine {
    system_path: String,
    available_requested_states: Option<Vec<requestedstate::RequestedState>>,
    caption: Option<String>,
    communication_status: Option<communicationstate::CommunicationStatus>,
    created: Option<DateTime<Utc>>,                          // From InstallDate
    creation_class_name: Option<String>,                     // Always set to "Msvm_ComputerSystem"
    dedicated: Option<Vec<dedicated::Dedicated>>,            // Always set to NotDedicated = 0
    description: Option<String>,                             // Will be "Microsoft Virtual Computer System" or "Microsoft Hosting Computer System"
    detailed_status: Option<detailedstatus::DetailedStatus>, // Complements PrimaryStatus
    enabled_default: Option<enableddefault::EnabledDefault>, // Default Enabled = 2 for a physical computer
    enabled_state: Option<enabledstate::EnabledState>,       // 2 is only for physical computer. There is no default for a VM
    enhanced_session_mode_state: Option<enhancedsessionmodestate::EnhancedSessionModeState>,
    health_state: Option<healthstate::HealthState>, // Default Ok = 5
    id: VmId,                                       // From Name
    identifying_descriptions: Option<Vec<String>>,  // Always set to null
    instance_id: Option<String>,
    last_successful_backup_time: Option<DateTime<Utc>>,
    name: Option<String>,                                       // From ElementName
    name_format: Option<String>,                                // Always set to null
    number_of_numa_nodes: Option<u16>,                          // Set to null for the management OS
    on_time: Option<Duration>,                                  // From OnTimeInMilliseconds
    operating_status: Option<operatingstatus::OperatingStatus>, // Null means not implemented
    operational_status: Option<operationalstatus::OperationalStatus>,
    other_dedicated_descriptions: Option<Vec<String>>,                                                    // Always set to null
    other_enabled_state: Option<String>,                                                                  // Must be null when EnabledState is not Other. Always set to null
    other_identifying_info: Option<Vec<String>>,                                                          // Always set to null
    power_management_capabilities: Option<Vec<powermanagementcapabilities::PowerManagementCapabilities>>, // Not used
    primary_owner_contact: Option<String>,                                                                // Always set to null
    primary_owner_name: Option<String>,                                                                   // Always set to null
    primary_status: Option<primarystatus::PrimaryStatus>,                                                 // Used in conjunction with DetailedStatus. Null indicates not implemented.
    process_id: Option<u32>,
    replication_mode: Option<replicationmode::ReplicationMode>,
    requested_state: Option<requestedstate::RequestedState>,
    reset_capability: Option<resetcapability::ResetCapability>, // Always set to Other = 1
    roles: Option<Vec<String>>,                                 // Always set to null
    status: Option<String>,                                     // Not used
    status_descriptions: Option<Vec<String>>,
    time_of_last_configuration_change: Option<DateTime<Utc>>,
    time_of_last_state_change: Option<DateTime<Utc>>,
    transitioning_to_state: Option<transitioningtostate::TransitioningToState>, // Not used
}

impl VirtualMachine {
    pub fn system_path(&self) -> &str {
        &self.system_path
    }

    pub fn available_requested_states(&self) -> Option<&[requestedstate::RequestedState]> {
        self.available_requested_states.as_deref()
    }
    pub fn caption(&self) -> Option<&str> {
        self.caption.as_deref()
    }
    pub fn communication_status(&self) -> Option<&communicationstate::CommunicationStatus> {
        self.communication_status.as_ref()
    }
    pub fn created(&self) -> Option<DateTime<Utc>> {
        self.created
    }
    pub fn creation_class_name(&self) -> Option<&str> {
        self.creation_class_name.as_deref()
    }
    pub fn dedicated(&self) -> Option<&[dedicated::Dedicated]> {
        self.dedicated.as_deref()
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn detailed_status(&self) -> Option<&detailedstatus::DetailedStatus> {
        self.detailed_status.as_ref()
    }
    pub fn enabled_default(&self) -> Option<&enableddefault::EnabledDefault> {
        self.enabled_default.as_ref()
    }
    pub fn enabled_state(&self) -> Option<&enabledstate::EnabledState> {
        self.enabled_state.as_ref()
    }
    pub fn enhanced_session_mode_state(&self) -> Option<&enhancedsessionmodestate::EnhancedSessionModeState> {
        self.enhanced_session_mode_state.as_ref()
    }
    pub fn health_state(&self) -> Option<&healthstate::HealthState> {
        self.health_state.as_ref()
    }
    pub fn id(&self) -> VmId {
        self.id
    }
    pub fn identifying_descriptions(&self) -> Option<&[String]> {
        self.identifying_descriptions.as_deref()
    }
    pub fn instance_id(&self) -> Option<&str> {
        self.instance_id.as_deref()
    }
    pub fn last_successful_backup_time(&self) -> Option<DateTime<Utc>> {
        self.last_successful_backup_time
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn name_format(&self) -> Option<&str> {
        self.name_format.as_deref()
    }
    pub fn number_of_numa_nodes(&self) -> Option<u16> {
        self.number_of_numa_nodes
    }
    pub fn on_time(&self) -> Option<Duration> {
        self.on_time
    }
    pub fn operating_status(&self) -> Option<&operatingstatus::OperatingStatus> {
        self.operating_status.as_ref()
    }
    pub fn operational_status(&self) -> Option<&operationalstatus::OperationalStatus> {
        self.operational_status.as_ref()
    }
    pub fn other_dedicated_descriptions(&self) -> Option<&[String]> {
        self.other_dedicated_descriptions.as_deref()
    }
    pub fn other_enabled_state(&self) -> Option<&str> {
        self.other_enabled_state.as_deref()
    }
    pub fn other_identifying_info(&self) -> Option<&[String]> {
        self.other_identifying_info.as_deref()
    }
    pub fn power_management_capabilities(&self) -> Option<&[powermanagementcapabilities::PowerManagementCapabilities]> {
        self.power_management_capabilities.as_deref()
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
        self.replication_mode.as_ref()
    }
    pub fn requested_state(&self) -> Option<&requestedstate::RequestedState> {
        self.requested_state.as_ref()
    }
    pub fn reset_capability(&self) -> Option<&resetcapability::ResetCapability> {
        self.reset_capability.as_ref()
    }
    pub fn roles(&self) -> Option<&[String]> {
        self.roles.as_deref()
    }
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }
    pub fn status_descriptions(&self) -> Option<&[String]> {
        self.status_descriptions.as_deref()
    }
    pub fn time_of_last_configuration_change(&self) -> Option<DateTime<Utc>> {
        self.time_of_last_configuration_change
    }
    pub fn time_of_last_state_change(&self) -> Option<DateTime<Utc>> {
        self.time_of_last_state_change
    }
    pub fn transitioning_to_state(&self) -> Option<&transitioningtostate::TransitioningToState> {
        self.transitioning_to_state.as_ref()
    }
}

impl TryFrom<computersystem::ComputerSystem> for VirtualMachine {
    type Error = Error;

    fn try_from(computer_system: computersystem::ComputerSystem) -> Result<Self, Self::Error> {
        Ok(Self {
            system_path: computer_system.__Path,
            available_requested_states: computer_system
                .AvailableRequestedStates
                .map(|states| {
                    states
                        .into_iter()
                        .map(requestedstate::RequestedState::try_from)
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| Error::InvalidField("AvailableRequestedStates"))
                })
                .transpose()?,
            caption: computer_system.Caption,
            communication_status: computer_system
                .CommunicationStatus
                .map(|value| communicationstate::CommunicationStatus::try_from(value).map_err(|_| Error::InvalidField("CommunicationStatus")))
                .transpose()?,
            created: computer_system.InstallDate.map(|v| v.0.with_timezone(&Utc)),
            creation_class_name: computer_system.CreationClassName, // Always set to "Msvm_ComputerSystem"
            dedicated: computer_system
                .Dedicated
                .map(|values| values.into_iter().map(dedicated::Dedicated::try_from).collect::<Result<Vec<_>, _>>().map_err(|_| Error::InvalidField("Dedicated")))
                .transpose()?, // Always set to NotDedicated = 0
            description: computer_system.Description,               // Will be "Microsoft Virtual Computer System" or "Microsoft Hosting Computer System"
            detailed_status: computer_system
                .DetailedStatus
                .map(|value| detailedstatus::DetailedStatus::try_from(value).map_err(|_| Error::InvalidField("DetailedStatus")))
                .transpose()?, // Complements PrimaryStatus
            enabled_default: computer_system
                .EnabledDefault
                .map(|value| enableddefault::EnabledDefault::try_from(value).map_err(|_| Error::InvalidField("EnabledDefault")))
                .transpose()?,
            enabled_state: computer_system
                .EnabledState
                .map(|value| enabledstate::EnabledState::try_from(value).map_err(|_| Error::InvalidField("EnabledState")))
                .transpose()?,
            enhanced_session_mode_state: computer_system
                .EnhancedSessionModeState
                .map(|value| enhancedsessionmodestate::EnhancedSessionModeState::try_from(value).map_err(|_| Error::InvalidField("EnhancedSessionModeState")))
                .transpose()?,
            health_state: computer_system.HealthState.map(|value| healthstate::HealthState::try_from(value).map_err(|_| Error::InvalidField("HealthState"))).transpose()?, // Default Ok = 5
            id: VmId::new(Uuid::parse_str(&computer_system.Name)?).ok_or(Error::InvalidField("Name"))?,
            identifying_descriptions: computer_system.IdentifyingDescriptions, // Always set to null
            instance_id: computer_system.InstanceID,
            last_successful_backup_time: computer_system.LastSuccessfulBackupTime.map(|v| v.0.with_timezone(&Utc)),
            name: computer_system.ElementName,
            name_format: computer_system.NameFormat,                 // Always set to null
            number_of_numa_nodes: computer_system.NumberOfNumaNodes, // Set to null for the management OS
            on_time: computer_system.OnTimeInMilliseconds.map(Duration::from_millis),
            operating_status: computer_system
                .OperatingStatus
                .map(|value| operatingstatus::OperatingStatus::try_from(value).map_err(|_| Error::InvalidField("OperatingStatus")))
                .transpose()?, // Null means not implemented
            operational_status: computer_system
                .OperationalStatus
                .map(|values| operationalstatus::OperationalStatus::from_values(values).map_err(|_| Error::InvalidField("OperationalStatus")))
                .transpose()?,
            other_dedicated_descriptions: computer_system.OtherDedicatedDescriptions, // Always set to null
            other_enabled_state: computer_system.OtherEnabledState,                   // Must be null when EnabledState is not Other. Always set to null
            other_identifying_info: computer_system.OtherIdentifyingInfo,             // Always set to null
            power_management_capabilities: computer_system
                .PowerManagementCapabilities
                .map(|values| {
                    values
                        .into_iter()
                        .map(powermanagementcapabilities::PowerManagementCapabilities::try_from)
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| Error::InvalidField("PowerManagementCapabilities"))
                })
                .transpose()?, // Not used
            primary_owner_contact: computer_system.PrimaryOwnerContact,               // Always set to null
            primary_owner_name: computer_system.PrimaryOwnerName,                     // Always set to null
            primary_status: computer_system
                .PrimaryStatus
                .map(|value| primarystatus::PrimaryStatus::try_from(value).map_err(|_| Error::InvalidField("PrimaryStatus")))
                .transpose()?, // Used in conjunction with DetailedStatus. Null indicates not implemented.
            process_id: computer_system.ProcessID,
            replication_mode: computer_system
                .ReplicationMode
                .map(|value| replicationmode::ReplicationMode::try_from(value).map_err(|_| Error::InvalidField("ReplicationMode")))
                .transpose()?,
            requested_state: computer_system
                .RequestedState
                .map(|value| requestedstate::RequestedState::try_from(value).map_err(|_| Error::InvalidField("RequestedState")))
                .transpose()?,
            reset_capability: computer_system
                .ResetCapability
                .map(|value| resetcapability::ResetCapability::try_from(value).map_err(|_| Error::InvalidField("ResetCapability")))
                .transpose()?, // Always set to Other = 1
            roles: computer_system.Roles,   // Always set to null
            status: computer_system.Status, // Not used
            status_descriptions: computer_system.StatusDescriptions,
            time_of_last_configuration_change: computer_system.TimeOfLastConfigurationChange.map(|v| v.0.with_timezone(&Utc)),
            time_of_last_state_change: computer_system.TimeOfLastStateChange.map(|v| v.0.with_timezone(&Utc)),
            transitioning_to_state: computer_system
                .TransitioningToState
                .map(|value| transitioningtostate::TransitioningToState::try_from(value).map_err(|_| Error::InvalidField("TransitioningToState")))
                .transpose()?, // Not used
        })
    }
}
