use super::*;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub(crate) struct VirtualSystemSnapshotService {
    available_requested_states: Option<Vec<AvailableRequestedState>>,
    caption: String,
    communication_status: Option<super::CommunicationStatus>,
    connection: wmi::WMIConnection,
    creation_class_name: String,
    description: String,
    detailed_status: Option<DetailedStatus>,
    element_name: String,
    enabled_default: EnabledState, // Always 2 = Enabled
    enabled_state: EnabledState,   // Always 2 = Enabled
    health_state: HealthState,     // Always set to 5 = Ok but is a range 0 to 30 where 30 is completely non-functional
    instance_id: Option<InstanceId>,
    install_date: chrono::DateTime<chrono::Utc>,
    name: String,
    operating_status: Option<OperatingStatus>,
    operational_status: Vec<OperationalStatus>, // Always set to 2 = OK
    other_enabled_state: Option<String>,
    path: String,
    primary_owner_contact: Option<String>, // Always set to null
    primary_owner_name: Option<String>,    // Always set to null
    primary_status: Option<PrimaryStatus>,
    requested_state: Option<RequestedState>, // Always set to 12 = Not Applicable
    start_mode: Option<String>,              // Always set to null
    started: bool,
    status: Option<String>, // Not used
    status_descriptions: Vec<String>,
    system_creation_class_name: String,
    system_name: String,
    time_of_last_state_change: chrono::DateTime<chrono::Utc>,
    transitioning_to_state: Option<TransitioningState>,
}

impl std::fmt::Display for VirtualSystemSnapshotService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VirtualSystemSnapshotService")
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

impl VirtualSystemSnapshotService {
    fn method(&self, method_name: &str) -> wmi::WMIResult<wmi::IWbemClassWrapper> {
        self.connection
            .get_object("Msvm_VirtualSystemSnapshotService")?
            .get_method(method_name)?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("{method_name} method signature not found").into()))
    }

    pub(crate) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<VirtualSystemSnapshotServiceOut>("SELECT * FROM Msvm_VirtualSystemSnapshotService WHERE __CLASS = 'Msvm_VirtualSystemSnapshotService'")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system snapshot service not found".into()))?;
        Self::from(service, connection).map_err(|error| wmi::WMIError::ConvertVariantError(error.to_string().into()))
    }

    fn from(output: VirtualSystemSnapshotServiceOut, connection: wmi::WMIConnection) -> Result<Self, serde::de::value::Error> {
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

    pub(crate) async fn apply(&self, snapshot: VirtualSystemSettingData) -> wmi::WMIResult<JobState> {
        let apply_snapshot_method_class = self.method("ApplySnapshot")?;
        log::debug!("ApplySnapshot requested for snapshot instance: {}", snapshot.path.as_str());
        let input = apply_snapshot_method_class.spawn_instance()?;
        input.put_property("SnapshotSettingData", snapshot.path.as_str()).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set SnapshotSettingData: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "ApplySnapshot", &input).await
    }

    pub(crate) async fn clear_state(&self, snapshot_setting_data: VirtualSystemSettingData) -> wmi::WMIResult<JobState> {
        let clear_snapshot_state_tree_method_class = self.method("ClearSnapshotState")?;
        let input = clear_snapshot_state_tree_method_class.spawn_instance()?;
        input
            .put_property("SnapshotSettingData", snapshot_setting_data.path.as_str())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set SnapshotSettingData: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "ClearSnapshotState", &input).await
    }

    pub(crate) async fn create(&self, affected_system: &VirtualMachine, snapshot_settings: Option<VirtualSystemSettingDataIn>, snapshot_type: SnapshotType, resulting_snapshot: Option<VirtualSystemSettingData>) -> wmi::WMIResult<VirtualSystemSettingData> {
        let create_snapshot_method_class = self.method("CreateSnapshot")?;
        let mut job_events = self.connection.async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let snapshot_setting_data_xml_string = snapshot_settings.as_ref().map(|rps| rps.to_xml()).transpose().map_err(|e| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {e}").into()))?.unwrap_or_default();
        let snapshot_type_value = u16::from(snapshot_type);
        let input = create_snapshot_method_class.spawn_instance()?;
        input.put_property("AffectedSystem", affected_system.path.as_str()).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSystem: {error}").into()))?;
        input.put_property("SnapshotSettings", snapshot_setting_data_xml_string).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set SnapshotSettings: {error}").into()))?;
        input.put_property("SnapshotType", snapshot_type_value).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set SnapshotType: {error}").into()))?;
        let resulting_snapshot_path = resulting_snapshot.as_ref().map(|snapshot| wmi::Variant::String(snapshot.path.as_str().to_owned())).unwrap_or(wmi::Variant::Null);
        input.put_property("ResultingSnapshot", resulting_snapshot_path).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ResultingSnapshot: {error}").into()))?;
        let result = self
            .connection
            .exec_method(&self.path, "CreateSnapshot", Some(&input))
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("CreateSnapshot WMI call failed: {error}").into()))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateSnapshot returned no output".into()))?
            .into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                let path = result
                    .resulting_snapshot
                    .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateSnapshot returned no resulting snapshot".into()))?;
                self.connection.get_object(path)?.into_desr::<VirtualSystemSettingData>()
            },
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateSnapshot returned no job".into()))?;
                let job = Job::wait(&self.connection, path, &mut job_events).await?;
                job.get_related("Msvm_VirtualSystemSettingData").await
            },
            return_value => {
                let return_value_message = method_return_value_description(return_value);
                Err(wmi::WMIError::ConvertVariantError(format!("CreateSnapshot failed: {return_value_message} ({return_value})").into()))
            },
        }
    }

    pub(crate) async fn destroy(&self, affected_snapshot: VirtualSystemSettingData) -> wmi::WMIResult<JobState> {
        let destroy_snapshot_method_class = self.method("DestroySnapshot")?;
        let input = destroy_snapshot_method_class.spawn_instance()?;
        input.put_property("AffectedSnapshot", affected_snapshot.path.as_str()).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSnapshot: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "DestroySnapshot", &input).await
    }

    pub(crate) async fn destroy_tree(&self, snapshot_setting_data: VirtualSystemSettingData) -> wmi::WMIResult<JobState> {
        let destroy_snapshot_tree_method_class = self.method("DestroySnapshotTree")?;
        let input = destroy_snapshot_tree_method_class.spawn_instance()?;
        input
            .put_property("SnapshotSettingData", snapshot_setting_data.path.as_str())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set SnapshotSettingData: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "DestroySnapshotTree", &input).await
    }

    pub(crate) async fn convert_to_reference_point(&self, affected_snapshot: VirtualSystemSettingData, reference_point_settings: Option<VirtualSystemReferencePointSettingDataIn>, resulting_reference_point: Option<VirtualSystemReferencePoint>) -> wmi::WMIResult<VirtualSystemReferencePoint> {
        let convert_to_reference_point_method_class = self.method("ConvertToReferencePoint")?;
        let mut job_events = self.connection.async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let reference_point_settings_xml_string = reference_point_settings.as_ref().map(|rps| rps.to_xml()).transpose().map_err(|e| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {e}").into()))?.unwrap_or_default();
        let input = convert_to_reference_point_method_class.spawn_instance()?;
        input.put_property("AffectedSnapshot", affected_snapshot.path.as_str()).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSnapshot: {error}").into()))?;
        input
            .put_property("ReferencePointSettings", reference_point_settings_xml_string)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointSettings: {error}").into()))?;
        if let Some(resulting_reference_point) = resulting_reference_point.as_ref() {
            input
                .put_property("ResultingReferencePoint", resulting_reference_point.path.as_str())
                .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ResultingReferencePoint: {error}").into()))?;
        }
        let result = self
            .connection
            .exec_method(&self.path, "ConvertToReferencePoint", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("ConvertToReferencePoint returned no output".into()))?
            .into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                let path = result
                    .resulting_reference_point
                    .ok_or_else(|| wmi::WMIError::ConvertVariantError("ConvertToReferencePoint returned no resulting reference point".into()))?;
                self.connection.get_object(path)?.into_desr::<VirtualSystemReferencePoint>()
            },
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("ConvertToReferencePoint returned no job".into()))?;
                let job = Job::wait(&self.connection, path, &mut job_events).await?;
                job.get_related("Msvm_VirtualSystemReferencePoint").await
            },
            return_value => {
                let description = method_return_value_description(return_value);
                Err(wmi::WMIError::ConvertVariantError(format!("ConvertToReferencePoint failed: {description} ({return_value})").into()))
            },
        }
    }
}

impl<'de> Deserialize<'de> for VirtualSystemSnapshotService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let output = VirtualSystemSnapshotServiceOut::deserialize(deserializer)?;
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).map_err(serde::de::Error::custom)?;
        Self::from(output, connection).map_err(serde::de::Error::custom)
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn creates_snapshot_for_configured_vm() {
        let _ = env_logger::try_init();
        let vm_name = std::env::var("HYPER_V_VM_NAME").expect("HYPER_V_VM_NAME must be set");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let computer_system = connection
            .raw_query::<super::ComputerSystemOut>("SELECT * FROM Msvm_ComputerSystem WHERE Caption = 'Virtual Machine'")
            .expect("Hyper-V virtual machines should be queryable")
            .into_iter()
            .find(|machine| machine.ElementName == vm_name)
            .expect("configured Hyper-V VM should be queryable");
        log::debug!("{computer_system:#}");
        let virtual_machine = VirtualMachine::from(computer_system, connection.clone()).expect("virtual machine WMI data should be valid");
        log::debug!("{virtual_machine:#}");
        let service = VirtualSystemSnapshotService::new(connection).expect("Hyper-V snapshot service should be available");
        let snapshot_setting_data = VirtualSystemSettingDataIn {
            classname: "Msvm_VirtualSystemSnapshotSettingData",
            properties: vec![
                Property {
                    name: "ConsistencyLevel".into(),
                    cim_type: "uint8".into(),
                    value: u8::from(ConsistencyLevel::Crash).to_string(),
                },
                Property {
                    name: "IgnoreNonSnapshottableDisks".into(),
                    cim_type: "boolean".into(),
                    value: true.to_string(),
                },
            ],
        };
        let virtual_system_setting_data = service
            .create(&virtual_machine, Some(snapshot_setting_data), SnapshotType::VendorSpecific(32768), None) // 32768 is a recovery snapshot used in backups
            .await
            .expect("snapshot should be created");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let job_state = service.destroy(virtual_system_setting_data).await.expect("snapshot should be destroyed");
        assert!(matches!(job_state, JobState::Completed));
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn creates_and_destroys_snapshot_tree_for_configured_vm() {
        let _ = env_logger::try_init();
        let vm_name = std::env::var("HYPER_V_VM_NAME").expect("HYPER_V_VM_NAME must be set");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let computer_system = connection
            .raw_query::<super::computersystemout::ComputerSystemOut>("SELECT * FROM Msvm_ComputerSystem WHERE Caption = 'Virtual Machine'")
            .expect("Hyper-V virtual machines should be queryable")
            .into_iter()
            .find(|machine| machine.ElementName == vm_name)
            .expect("configured Hyper-V VM should be queryable");
        let virtual_machine = VirtualMachine::from(computer_system, connection.clone()).expect("virtual machine WMI data should be valid");
        let service = VirtualSystemSnapshotService::new(connection).expect("Hyper-V snapshot service should be available");
        let mut snapshots = Vec::with_capacity(5);

        for _ in 0..5 {
            snapshots.push(
                service
                    .create(
                        &virtual_machine,
                        Some(VirtualSystemSettingDataIn {
                            classname: "Msvm_VirtualSystemSnapshotSettingData",
                            properties: vec![
                                Property {
                                    name: "ConsistencyLevel".into(),
                                    cim_type: "uint8".into(),
                                    value: u8::from(ConsistencyLevel::Crash).to_string(),
                                },
                                Property {
                                    name: "IgnoreNonSnapshottableDisks".into(),
                                    cim_type: "boolean".into(),
                                    value: true.to_string(),
                                },
                            ],
                        }),
                        SnapshotType::Full, // TODO: Cannot delete SnapshotType::VendorSpecific(32768)
                        None,
                    )
                    .await
                    .expect("snapshot should be created"),
            );
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let job_state = service.destroy_tree(snapshots.remove(3)).await.expect("snapshot tree should be destroyed from the third descendant");
        assert!(matches!(job_state, JobState::Completed));

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let job_state = service.destroy_tree(snapshots.remove(0)).await.expect("snapshot tree should be destroyed from the root");
        assert!(matches!(job_state, JobState::Completed));
    }
}
