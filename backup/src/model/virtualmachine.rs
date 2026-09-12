use std::time::Duration;

use crate::infra::*;
use crate::model::VirtualMachineId;

#[derive(Clone, Debug)]
pub struct VirtualMachine {
    available_requested_states: Vec<RequestedState>,
    caption: String,
    communication_status: Option<CommunicationStatus>,
    connection: wmi::WMIConnection,
    creation_class_name: String,    // Always set to "Msvm_ComputerSystem"
    dedicated: Vec<Dedicated>,  // Always set to NotDedicated = 0
    description: String,    // Will be "Microsoft Virtual Computer System" or "Microsoft Hosting Computer System"
    detailed_status: Option<DetailedStatus>, // Complements PrimaryStatus
    pub(crate) element_name: String,
    enabled_default: EnabledDefault, // Default Enabled = 2 for a physical computer
    enabled_state: EnabledState,    // 2 is only for physical computer. There is no default for a VM
    enhanced_session_mode_state: EnhancedSessionModeState,
    health_state: HealthState, // Default Ok = 5
    identifying_descriptions: Vec<String>,  // Always set to null
    install_date: chrono::DateTime<chrono::Utc>,
    instance_id: Option<String>,
    last_successful_backup_time: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) name: VirtualMachineId,
    name_format: Option<String>,    // Always set to null
    number_of_numa_nodes: u16,  // Set to null for the management OS
    on_time: Duration,  // From OnTimeInMilliseconds
    operating_status: Option<OperatingStatus>, // Null means not implemented
    operational_status: OperationalStatus,
    other_dedicated_descriptions: Vec<String>,  // Always set to null
    other_enabled_state: Option<String>,    // Must be null when EnabledState is not Other. Always set to null
    other_identifying_info: Vec<String>,    // Always set to null
    pub(crate) path: Path,
    power_management_capabilities: Vec<PowerManagementCapabilities>, // Not used
    primary_owner_contact: Option<String>,  // Always set to null
    primary_owner_name: Option<String>, // Always set to null
    primary_status: Option<PrimaryStatus>,  // Used in conjunction with DetailedStatus. Null indicates not implemented.
    process_id: Option<u32>,
    replication_mode: ReplicationMode,
    requested_state: RequestedState,
    reset_capability: ResetCapability, // Always set to Other = 1
    roles: Vec<String>, // Always set to null
    status: String, // Not used
    status_descriptions: Vec<String>,
    time_of_last_configuration_change: chrono::DateTime<chrono::Utc>,
    time_of_last_state_change: chrono::DateTime<chrono::Utc>,
    transitioning_to_state: Option<TransitioningToState>, // Not used
}

impl std::fmt::Display for VirtualMachine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VirtualMachine")
            .field("available_requested_states", &self.available_requested_states)
            .field("caption", &self.caption)
            .field("communication_status", &self.communication_status)
            .field("creation_class_name", &self.creation_class_name)
            .field("dedicated", &self.dedicated)
            .field("description", &self.description)
            .field("detailed_status", &self.detailed_status)
            .field("element_name", &self.element_name)
            .field("enabled_default", &self.enabled_default)
            .field("enabled_state", &self.enabled_state)
            .field("enhanced_session_mode_state", &self.enhanced_session_mode_state)
            .field("health_state", &self.health_state)
            .field("identifying_descriptions", &self.identifying_descriptions)
            .field("install_date", &self.install_date)
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
    pub(crate) fn from(computer_system: ComputerSystemOut, connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let path = computer_system.path.clone();
        let name = uuid::Uuid::parse_str(&computer_system.Name)
            .map(VirtualMachineId::from)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Invalid virtual machine name: {error}").into()))?;
        let operational_status: OperationalStatus = computer_system
            .OperationalStatus
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("OperationalStatus cannot be empty".into()))?
            .try_into()
            .map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?;

        Ok(Self {
            available_requested_states: computer_system.AvailableRequestedStates.into_iter().map(TryInto::try_into).collect::<Result<_, _>>().map_err(|error: String| wmi::WMIError::ConvertVariantError(error.into()))?,
            caption: computer_system.Caption,
            communication_status: computer_system.CommunicationStatus.map(TryInto::try_into).transpose().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            connection,
            creation_class_name: computer_system.CreationClassName,
            dedicated: computer_system.Dedicated.into_iter().map(TryInto::try_into).collect::<Result<_, _>>().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            description: computer_system.Description,
            detailed_status: computer_system.DetailedStatus.map(TryInto::try_into).transpose().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            element_name: computer_system.ElementName,
            enabled_default: computer_system.EnabledDefault.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            enabled_state: computer_system.EnabledState.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            enhanced_session_mode_state: computer_system.EnhancedSessionModeState.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            health_state: computer_system.HealthState.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            identifying_descriptions: computer_system.IdentifyingDescriptions,
            install_date: computer_system.InstallDate.0.with_timezone(&chrono::Utc),
            instance_id: computer_system.InstanceID,
            last_successful_backup_time: computer_system.LastSuccessfulBackupTime.map(|value| value.0.with_timezone(&chrono::Utc)),
            name,
            name_format: computer_system.NameFormat,
            number_of_numa_nodes: computer_system.NumberOfNumaNodes,
            on_time: Duration::from_millis(computer_system.OnTimeInMilliseconds),
            operating_status: computer_system.OperatingStatus.map(TryInto::try_into).transpose().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            operational_status,
            other_dedicated_descriptions: computer_system.OtherDedicatedDescriptions,
            other_enabled_state: computer_system.OtherEnabledState,
            other_identifying_info: computer_system.OtherIdentifyingInfo,
            path: path.into(),
            power_management_capabilities: computer_system.PowerManagementCapabilities.into_iter().map(TryInto::try_into).collect::<Result<_, _>>().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            primary_owner_contact: computer_system.PrimaryOwnerContact,
            primary_owner_name: computer_system.PrimaryOwnerName,
            primary_status: computer_system.PrimaryStatus.map(TryInto::try_into).transpose().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            process_id: computer_system.ProcessID,
            replication_mode: computer_system.ReplicationMode.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            requested_state: computer_system.RequestedState.try_into().map_err(|error: String| wmi::WMIError::ConvertVariantError(error.into()))?,
            reset_capability: computer_system.ResetCapability.try_into().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
            roles: computer_system.Roles,
            status: computer_system.Status,
            status_descriptions: computer_system.StatusDescriptions,
            time_of_last_configuration_change: computer_system.TimeOfLastConfigurationChange.0.with_timezone(&chrono::Utc),
            time_of_last_state_change: computer_system.TimeOfLastStateChange.0.with_timezone(&chrono::Utc),
            transitioning_to_state: computer_system.TransitioningToState.map(TryInto::try_into).transpose().map_err(|error: &'static str| wmi::WMIError::ConvertVariantError(error.into()))?,
        })
    }

    pub(super) async fn get_snapshots(&self) -> Vec<VirtualSystemSettingData> {
        let query = format!("ASSOCIATORS OF {{{}}} WHERE AssocClass = Msvm_SettingsDefineState", self.path.as_str());
        self.connection.async_raw_query::<VirtualSystemSettingData>(&query)
            .await.unwrap_or_default()
            .into_iter()
            .filter(VirtualSystemSettingData::is_snapshot)
            .collect()
    }

    pub(super) async fn get_reference_points(&self) -> Vec<VirtualSystemReferencePoint> {
        let query = format!("ASSOCIATORS OF {{{}}} WHERE AssocClass = Msvm_ReferencePointOfVirtualSystem", self.path.as_str());
        self.connection.async_raw_query::<VirtualSystemReferencePoint>(&query).await.unwrap_or_default()
    }
}
