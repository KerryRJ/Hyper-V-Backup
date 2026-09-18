use super::*;

#[derive(Debug)]
pub(crate) struct ImageManagementService {
    pub(crate) path: String,
    available_requested_states: Option<Vec<AvailableRequestedState>>,
    caption: String,
    communication_status: Option<CommunicationStatus>,
    connection: wmi::WMIConnection,
    creation_class_name: String,
    description: String,
    detailed_status: Option<DetailedStatus>,
    element_name: String,
    enabled_default: EnabledState,
    enabled_state: EnabledState,
    health_state: HealthState,
    instance_id: Option<InstanceId>,
    install_date: chrono::DateTime<chrono::Utc>,
    name: String,
    operating_status: Option<OperatingStatus>,
    operational_status: Vec<OperationalStatus>,
    other_enabled_state: Option<String>,
    primary_owner_contact: Option<String>,
    primary_owner_name: Option<String>,
    primary_status: Option<PrimaryStatus>,
    requested_state: Option<RequestedState>,
    start_mode: Option<String>,
    started: bool,
    status: Option<String>,
    status_descriptions: Vec<String>,
    system_creation_class_name: String,
    system_name: String,
    time_of_last_state_change: chrono::DateTime<chrono::Utc>,
    transitioning_to_state: Option<TransitioningToState>,
}

impl ImageManagementService {
    pub(crate) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<ImageManagementServiceOut>("SELECT * FROM Msvm_ImageManagementService WHERE __CLASS = 'Msvm_ImageManagementService'")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Image management service not found".into()))?;
        Self::from(service, connection).map_err(|error| wmi::WMIError::ConvertVariantError(error.to_string().into()))
    }

    fn from(output: ImageManagementServiceOut, connection: wmi::WMIConnection) -> Result<Self, serde::de::value::Error> {
        Ok(Self {
            path: output.Path,
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

    pub(crate) async fn get_virtual_hard_disk_setting_data(&self, path: impl AsRef<str>) -> wmi::WMIResult<VirtualHardDiskSettingData> {
        let path = path.as_ref().to_owned();
        loop {
            let method = self
                .connection
                .get_object("Msvm_ImageManagementService")?
                .get_method("GetVirtualHardDiskSettingData")?
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("GetVirtualHardDiskSettingData method signature not found".into()))?;
            let input = method.spawn_instance()?;
            input.put_property("Path", path.as_str())?;
            let job_query = "SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'";
            let mut job_events = self.connection.async_raw_notification::<ConcreteJobModificationEvent>(job_query)?;
            let output = self
                .connection
                .exec_method(&self.path, "GetVirtualHardDiskSettingData", Some(&input))?
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("GetVirtualHardDiskSettingData returned no output".into()))?;
            let return_value: u32 = output.get_property("ReturnValue").and_then(TryInto::try_into)?;
            match return_value {
                0 => return match output.get_property("SettingData")? {
                    wmi::Variant::String(setting_data) => VirtualHardDiskSettingData::from_embedded_xml(&setting_data).map_err(|error| wmi::WMIError::ConvertVariantError(format!("GetVirtualHardDiskSettingData setting data deserialization failed: {error}").into())),
                    value => Err(wmi::WMIError::ConvertVariantError(format!("GetVirtualHardDiskSettingData returned unexpected SettingData value: {value:?}").into())),
                },
                4096 => {
                    let job_path = match output.get_property("Job")? {
                        wmi::Variant::String(path) => path,
                        value => return Err(wmi::WMIError::ConvertVariantError(format!("GetVirtualHardDiskSettingData returned unexpected Job value: {value:?}").into())),
                    };
                    Job::wait(&self.connection, job_path, &mut job_events).await?;
                }
                value => return Err(wmi::WMIError::ConvertVariantError(format!("GetVirtualHardDiskSettingData failed: {} ({value})", method_return_value_description(value)).into())),
            }
        }
    }
}
