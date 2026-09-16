use std::path::PathBuf;

use super::*;
use serde::{Deserialize, Deserializer};

pub(crate) struct VirtualSystemManagementService {
    available_requested_states: Option<Vec<AvailableRequestedState>>,
    caption: String,
    communication_status: Option<super::CommunicationStatus>,
    connection: wmi::WMIConnection,
    creation_class_name: String,
    description: String,
    detailed_status: Option<DetailedStatus>,
    element_name: String,
    enabled_default: EnabledState,   // Always 2 = Enabled
    enabled_state: EnabledState, // Always 2 = Enabled
    health_state: HealthState,   // Always set to 5 = Ok but is a range 0 to 30 where 30 is completely non-functional
    instance_id: Option<InstanceId>,
    install_date: chrono::DateTime<chrono::Utc>,
    name: String,
    operating_status: Option<OperatingStatus>,
    operational_status: Vec<OperationalStatus>,  // Always set to 2 = OK
    other_enabled_state: Option<String>,
    path: String,
    primary_owner_contact: Option<String>,    // Always set to null
    primary_owner_name: Option<String>, // Always set to null
    primary_status: Option<PrimaryStatus>,
    requested_state: Option<RequestedState>, // Always set to 12 = Not Applicable
    start_mode: Option<String>,  // Always set to null
    started: bool,
    status: Option<String>,  // Not used
    status_descriptions: Vec<String>,
    system_creation_class_name: String,
    system_name: String,
    time_of_last_state_change: chrono::DateTime<chrono::Utc>,
    transitioning_to_state: Option<TransitioningState>,
}

impl std::fmt::Display for VirtualSystemManagementService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VirtualSystemManagementService")
            .field("available_requested_states", &self.available_requested_states)
            .field("caption", &self.caption)
            .field("communication_status", &self.communication_status)
            .field("creation_class_name", &self.creation_class_name)
            .field("description", &self.description)
            .field("detailed_status", &self.detailed_status)
            .field("element_name", &self.element_name)
            .field("enabled_default", &self.enabled_default)
            .field("enabled_state", &self.enabled_state)
            .field("health_state", &self.health_state)
            .field("instance_id", &self.instance_id)
            .field("install_date", &self.install_date)
            .field("name", &self.name)
            .field("operating_status", &self.operating_status)
            .field("operational_status", &self.operational_status)
            .field("other_enabled_state", &self.other_enabled_state)
            .field("path", &self.path)
            .field("primary_owner_contact", &self.primary_owner_contact)
            .field("primary_owner_name", &self.primary_owner_name)
            .field("primary_status", &self.primary_status)
            .field("requested_state", &self.requested_state)
            .field("start_mode", &self.start_mode)
            .field("started", &self.started)
            .field("status", &self.status)
            .field("status_descriptions", &self.status_descriptions)
            .field("system_creation_class_name", &self.system_creation_class_name)
            .field("system_name", &self.system_name)
            .field("time_of_last_state_change", &self.time_of_last_state_change)
            .field("transitioning_to_state", &self.transitioning_to_state)
            .finish()
    }
}

impl VirtualSystemManagementService {
    pub(crate) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<VirtualSystemManagementServiceOut>("SELECT * FROM Msvm_VirtualSystemManagementService WHERE __CLASS = 'Msvm_VirtualSystemManagementService'")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system management service not found".into()))?;
        Self::from(service, connection).map_err(|error| wmi::WMIError::ConvertVariantError(error.to_string().into()))
    }

    fn from(output: VirtualSystemManagementServiceOut, connection: wmi::WMIConnection) -> Result<Self, serde::de::value::Error> {
        Ok(Self {
            available_requested_states: output.AvailableRequestedStates.map(|values| values.into_iter().map(TryInto::try_into).collect()).transpose().map_err(serde::de::Error::custom)?,
            caption: output.Caption,
            communication_status: output.CommunicationStatus.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            connection,
            creation_class_name: output.CreationClassName,
            description: output.Description,
            detailed_status: output.DetailedStatus.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            element_name: output.ElementName,
            enabled_default: output.EnabledDefault.ok_or_else(|| serde::de::Error::custom("EnabledDefault cannot be null"))?.try_into().map_err(serde::de::Error::custom)?,
            enabled_state: output.EnabledState.ok_or_else(|| serde::de::Error::custom("EnabledState cannot be null"))?.try_into().map_err(serde::de::Error::custom)?,
            health_state: output.HealthState.ok_or_else(|| serde::de::Error::custom("HealthState cannot be null"))?.try_into().map_err(serde::de::Error::custom)?,
            instance_id: output.InstanceID.map(Into::into),
            install_date: output.InstallDate.0.with_timezone(&chrono::Utc),
            name: output.Name,
            operating_status: output.OperatingStatus.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            operational_status: output.OperationalStatus.into_iter().map(TryInto::try_into).collect::<Result<_, _>>().map_err(serde::de::Error::custom)?,
            other_enabled_state: output.OtherEnabledState,
            path: output.Path,
            primary_owner_contact: output.PrimaryOwnerContact,
            primary_owner_name: output.PrimaryOwnerName,
            primary_status: output.PrimaryStatus.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            requested_state: output.RequestedState.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            start_mode: output.StartMode,
            started: output.Started,
            status: output.Status,
            status_descriptions: output.StatusDescriptions,
            system_creation_class_name: output.SystemCreationClassName,
            system_name: output.SystemName,
            time_of_last_state_change: output.TimeOfLastStateChange.0.with_timezone(&chrono::Utc),
            transitioning_to_state: output.TransitioningToState.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
        })
    }    

    pub(crate) async fn export_system_definition(&self, computer_system: VirtualMachine, export_directory: PathBuf, export_setting_data: Option<VirtualSystemExportSettingDataIn>) -> wmi::WMIResult<JobState> {
        let export_system_definition_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemManagementService")?
            .get_method("ExportSystemDefinition")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportSystemDefinition method signature not found".into()))?;
        let export_setting_data_xml_string = export_setting_data
            .as_ref()
            .map(VirtualSystemExportSettingDataIn::to_xml)
            .transpose()
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {error}").into()))?
            .unwrap_or_default();
        let input = export_system_definition_method_class.spawn_instance()?;
        input
            .put_property("ComputerSystem", computer_system.path.as_str())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ComputerSystem: {error}").into()))?;
        input
            .put_property("ExportDirectory", export_directory.to_str().ok_or_else(|| wmi::WMIError::ConvertVariantError("Invalid export directory path".into()))?)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportDirectory: {error}").into()))?;
        input
            .put_property("ExportSettingData", export_setting_data_xml_string)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportSettingData: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "ExportSystemDefinition", &input).await
    }
}

impl<'de> Deserialize<'de> for VirtualSystemManagementService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let output = VirtualSystemManagementServiceOut::deserialize(deserializer)?;
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).map_err(serde::de::Error::custom)?;
        Self::from(output, connection).map_err(serde::de::Error::custom)
    }
}