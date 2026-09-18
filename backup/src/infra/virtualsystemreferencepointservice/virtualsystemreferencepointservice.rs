use super::super::*;
use super::VirtualSystemReferencePointServiceOut;
use std::path::PathBuf;

pub(crate) struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    path: String,
}

impl VirtualSystemReferencePointService {
    fn method(&self, method_name: &str) -> wmi::WMIResult<wmi::IWbemClassWrapper> {
        self.connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method(method_name)?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("{method_name} method signature not found").into()))
    }

    pub(crate) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<VirtualSystemReferencePointServiceOut>("SELECT * FROM Msvm_VirtualSystemReferencePointService WHERE __CLASS = 'Msvm_VirtualSystemReferencePointService'")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system reference point service not found".into()))?;
        if service.class_name != "Msvm_VirtualSystemReferencePointService" {
            return Err(wmi::WMIError::ConvertVariantError(format!("Unexpected reference point service class: {}", service.class_name).into()));
        }
        Ok(Self { connection, path: service.path })
    }

    pub(crate) async fn create(&self, affected_system: &VirtualMachine, reference_point_settings: Option<VirtualSystemReferencePointSettingDataIn>, reference_point_type: super::ReferencePointTypeIn, resulting_reference_point: Option<&VirtualSystemReferencePoint>) -> wmi::WMIResult<VirtualSystemReferencePoint> {
        let method = self.method("CreateReferencePoint")?;
        let mut job_events = self.connection.async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let reference_point_setting_data_xml_string = reference_point_settings.as_ref().map(|rps| rps.to_xml()).transpose().map_err(|e| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {e}").into()))?.unwrap_or_default();
        let reference_point_type_value = u16::from(&reference_point_type);
        let input = method.spawn_instance()?;
        input.put_property("AffectedSystem", affected_system.path.as_str()).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSystem: {error}").into()))?;
        input
            .put_property("ReferencePointSettings", reference_point_setting_data_xml_string)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointSettings: {error}").into()))?;
        input.put_property("ReferencePointType", reference_point_type_value).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointType: {error}").into()))?;
        let resulting_reference_point_path = resulting_reference_point.as_ref().map(|reference_point| wmi::Variant::String(reference_point.path.as_str().to_owned())).unwrap_or(wmi::Variant::Null);
        input
            .put_property("ResultingReferencePoint", resulting_reference_point_path)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ResultingReferencePoint: {error}").into()))?;
        let output = match self.connection.exec_method(&self.path, "CreateReferencePoint", Some(&input)) {
            Ok(Some(output)) => output,
            Ok(None) => return Err(wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no output".into())),
            Err(error) => return Err(wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint WMI call failed: {error}").into())),
        };
        let result = output.into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                let path = result.resulting_reference_point.ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no resulting reference point".into()))?;
                self.connection.get_object(path)?.into_desr::<VirtualSystemReferencePoint>()
            }
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no job".into()))?;
                let job = Job::wait(&self.connection, path, &mut job_events).await?;
                job.get_related("Msvm_VirtualSystemReferencePoint").await
            }
            return_value => {
                let return_value_message = method_return_value_description(return_value);
                Err(wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint failed: {return_value_message} ({return_value})").into()))
            }
        }
    }

    pub(crate) async fn export(&self, reference_point: &VirtualSystemReferencePoint, export_directory: PathBuf, export_setting_data: VirtualSystemReferencePointSettingDataIn) -> wmi::WMIResult<JobState> {
        let method = self.method("ExportReferencePoint")?;
        let input = method.spawn_instance()?;
        let setting_data = export_setting_data.to_xml().map_err(|error| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {error}").into()))?;
        input
            .put_property("AffectedReferencePoint", reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        input
            .put_property("ExportDirectory", export_directory.to_str().ok_or_else(|| wmi::WMIError::ConvertVariantError("Invalid export directory path".into()))?)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportDirectory: {error}").into()))?;
        input.put_property("ExportSettingData", setting_data).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportSettingData: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "ExportReferencePoint", &input).await
    }

    pub(crate) async fn destroy(&self, affected_reference_point: &VirtualSystemReferencePoint) -> wmi::WMIResult<JobState> {
        let method = self.method("DestroyReferencePoint")?;
        let input = method.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", affected_reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "DestroyReferencePoint", &input).await
    }

    pub(crate) async fn remove_associated_data(&self, affected_reference_point: &VirtualSystemReferencePoint) -> wmi::WMIResult<JobState> {
        let method = self.method("RemoveAssociatedData")?;
        let input = method.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", affected_reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        Job::execute_method(&self.connection, &self.path, "RemoveAssociatedData", &input).await
    }

    pub(crate) async fn import_metadata(&self, _affected_system: &VirtualMachine, _config_file_path: PathBuf, _runtime_state_file_path: PathBuf) -> wmi::WMIResult<VirtualMachine> {
        Err(wmi::WMIError::ConvertVariantError("ImportMetadata is not implemented".into()))
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    async fn configured_vm(connection: &wmi::WMIConnection) -> VirtualMachine {
        let vm_name = std::env::var("HYPER_V_VM_NAME").expect("HYPER_V_VM_NAME must be set");
        log::info!("Resolving configured Hyper-V VM: {vm_name}");
        let computer_system = connection
            .raw_query::<super::computersystemout::ComputerSystemOut>("SELECT * FROM Msvm_ComputerSystem WHERE Caption = 'Virtual Machine'")
            .expect("Hyper-V virtual machines should be queryable")
            .into_iter()
            .find(|machine| machine.ElementName == vm_name)
            .expect("configured Hyper-V VM should be queryable");
        log::info!("Resolved configured Hyper-V VM: {}", computer_system.path);
        VirtualMachine::from(computer_system, connection.clone()).expect("virtual machine WMI data should be valid")
    }

    fn reference_point_settings() -> VirtualSystemReferencePointSettingDataIn {
        VirtualSystemReferencePointSettingDataIn {
            classname: "Msvm_VirtualSystemReferencePointSettingData",
            properties: vec![Property {
                name: "ConsistencyLevel".into(),
                cim_type: "uint8".into(),
                value: u8::from(ConsistencyLevel::Crash).to_string(),
            }],
        }
    }

    async fn create_reference_point(service: &VirtualSystemReferencePointService, connection: &wmi::WMIConnection) -> VirtualSystemReferencePoint {
        log::info!("Creating reference point");
        let reference_point = service.create(&configured_vm(connection).await, Some(reference_point_settings()), ReferencePointTypeIn::Rct, None).await.expect("reference point should be created");
        log::info!("Reference point created: {}", reference_point.path.as_str());
        log::info!("Waiting five seconds before the next reference-point operation");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        reference_point
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn creates_reference_point_for_configured_vm() {
        // TODO: This always returns Not supported
        let _ = env_logger::try_init();
        log::info!("Starting creates_reference_point_for_configured_vm");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        log::info!("Connected to Hyper-V WMI namespace: {}", HYPER_V_NAMESPACE);
        let service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        log::info!("Resolved Hyper-V virtual system reference-point service");
        let reference_point = create_reference_point(&service, &connection).await;
        log::info!("Destroying reference point created by create test");
        let job_state = service.destroy(&reference_point).await.expect("reference point should be destroyed");
        log::info!("Reference-point destroy returned state: {job_state:?}");
        assert!(matches!(job_state, JobState::Completed));
        log::info!("Finished creates_reference_point_for_configured_vm");
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn destroys_reference_point_for_configured_vm() {
        let _ = env_logger::try_init();
        log::info!("Starting destroys_reference_point_for_configured_vm");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        let reference_point = create_reference_point(&service, &connection).await;
        log::info!("Destroying reference point: {}", reference_point.path.as_str());
        let job_state = service.destroy(&reference_point).await.expect("reference point should be destroyed");
        assert!(matches!(job_state, JobState::Completed));
        log::info!("Finished destroys_reference_point_for_configured_vm");
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn removes_associated_data_for_configured_vm() {
        let _ = env_logger::try_init();
        log::info!("Starting removes_associated_data_for_configured_vm");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        let service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        let reference_point = create_reference_point(&service, &connection).await;
        log::info!("Removing associated data from reference point: {}", reference_point.path.as_str());
        let job_state = service.remove_associated_data(&reference_point).await.expect("associated data should be removed");
        assert!(matches!(job_state, JobState::Completed));
        let job_state = service.destroy(&reference_point).await.expect("reference point should be destroyed");
        assert!(matches!(job_state, JobState::Completed));
        log::info!("Finished removes_associated_data_for_configured_vm");
    }
}
