
use std::path::PathBuf;
use super::*;

pub(crate) struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    path: String,
}

impl VirtualSystemReferencePointService {
    pub(crate) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<VirtualSystemReferencePointServiceOut>("SELECT * FROM Msvm_VirtualSystemReferencePointService")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system reference point service not found".into()))?;
        Ok(Self { connection, path: service.path })
    }

    pub(crate) async fn create(&self, affected_system: &VirtualMachine, reference_point_settings: Option<VirtualSystemReferencePointSettingDataIn>, reference_point_type: ReferencePointType, resulting_reference_point: Option<&VirtualSystemReferencePoint>) -> wmi::WMIResult<VirtualSystemReferencePoint> {
        let create_reference_point_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("CreateReferencePoint")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint method signature not found".into()))?;
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let reference_point_setting_data_xml_string = reference_point_settings.as_ref().map(|rps| rps.to_xml())
            .transpose()
            .map_err(|e| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {e}").into()))?
            .unwrap_or_default();
        let reference_point_type_value = u16::from(&reference_point_type);
        let input = create_reference_point_method_class.spawn_instance()?;
        input
            .put_property("AffectedSystem", affected_system.path.as_str())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSystem: {error}").into()))?;
        input
            .put_property("ReferencePointSettings", reference_point_setting_data_xml_string)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointSettings: {error}").into()))?;
        input
            .put_property("ReferencePointType", reference_point_type_value)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointType: {error}").into()))?;
        let resulting_reference_point_path = resulting_reference_point
            .as_ref()
            .map(|reference_point| wmi::Variant::String(reference_point.path.as_str().to_owned()))
            .unwrap_or(wmi::Variant::Null);
        input
            .put_property("ResultingReferencePoint", resulting_reference_point_path)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ResultingReferencePoint: {error}").into()))?;
        let result = self
            .connection
            .exec_method(&self.path, "CreateReferencePoint", Some(&input))
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint WMI call failed: {error}").into()))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no output".into()))?
            .into_desr::<MethodResult>()?;
        if result.return_value != 0 && result.return_value != 4096 {
            let return_value_message =
                match result.return_value {
                    1 => "Not supported",
                    2 => "Failed",
                    3 => "Timeout",
                    4 => "Invalid parameter",
                    5 => "Invalid state",
                    6 => "Invalid type",
                    7..=4095 => "DMTF reserved",
                    4097..=32767 => "Method reserved",
                    32768 => "Failed",
                    32769 => "Access denied",
                    32770 => "Not supported",
                    32771 => "Status is unknown",
                    32772 => "Timeout",
                    32773 => "Invalid parameter",
                    32774 => "System is in use",
                    32775 => "Invalid state for this operation",
                    32776 => "Incorrect data type",
                    32777 => "System is not available",
                    32778 => "Out of memory",
                    32779..=65535 => "Vendor specific",
                    _ => "Unknown",
                };
            return Err(wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint failed: {return_value_message} ({})", result.return_value).into()));
        }
        let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no job".into()))?;
        let job = Job::wait(&self.connection, path, &mut job_events).await?;
        job.get_related("Msvm_VirtualSystemReferencePoint").await
    }

    pub(crate) async fn export(&self, reference_point: &VirtualSystemReferencePoint, export_directory: PathBuf, export_setting_data: VirtualSystemReferencePointSettingDataIn) -> wmi::WMIResult<JobState> {
        let export_reference_point_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("ExportReferencePoint")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportReferencePoint method signature not found".into()))?;
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let reference_point_setting_data_xml_string = export_setting_data
            .to_xml()
            .map_err(|e| wmi::WMIError::ConvertVariantError(format!("XML Gen Failed: {e}").into()))?;
        let input = export_reference_point_method_class.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        input
            .put_property("ExportDirectory", export_directory.to_str().ok_or_else(|| wmi::WMIError::ConvertVariantError("Invalid export directory path".into()))?)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportDirectory: {error}").into()))?;
        input
            .put_property("ExportSettingData", reference_point_setting_data_xml_string)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ExportSettingData: {error}").into()))?;
        let result = self
            .connection
            .exec_method(&self.path, "ExportReferencePoint", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportReferencePoint returned no output".into()))?
            .into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                log::debug!("ExportReferencePoint completed synchronously");
                return Ok(JobState::Completed);
            },
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportReferencePoint returned no job".into()))?;
                log::debug!("Waiting for ExportReferencePoint job: {path}");
                let job_state = Job::wait(&self.connection, path, &mut job_events).await?.job_state;
                log::debug!("ExportReferencePoint job completed with state: {job_state:?}");
                return Ok(job_state);
            },
            return_value => {
                let description = match return_value {
                    32768 => "Failed",
                    32769 => "Access denied",
                    32770 => "Not supported",
                    32771 => "Status is unknown",
                    32772 => "Timeout",
                    32773 => "Invalid parameter",
                    32774 => "System is in use",
                    32775 => "Invalid state for this operation",
                    32776 => "Incorrect data type",
                    32777 => "System is not available",
                    32778 => "Out of memory",
                    _ => "Unknown",
                };
                Err(wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint failed: {description} ({return_value})").into()))
            }
        }
    }

    pub(crate) async fn destroy(&self, affected_reference_point: VirtualSystemReferencePoint) -> wmi::WMIResult<JobState> {
        let destroy_reference_point_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("DestroyReferencePoint")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint method signature not found".into()))?;
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let input = destroy_reference_point_method_class.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", affected_reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        let result = self
            .connection
            .exec_method(&self.path, "DestroyReferencePoint", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no output".into()))?
            .into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                log::debug!("DestroyReferencePoint completed synchronously");
                return Ok(JobState::Completed);
            },
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no job".into()))?;
                log::debug!("Waiting for DestroyReferencePoint job: {path}");
                let job_state = Job::wait(&self.connection, path, &mut job_events).await?.job_state;
                log::debug!("DestroyReferencePoint job completed with state: {job_state:?}");
                return Ok(job_state);
            },
            return_value => {
                let description = match return_value {
                    32768 => "Failed",
                    32769 => "Access denied",
                    32770 => "Not supported",
                    32771 => "Status is unknown",
                    32772 => "Timeout",
                    32773 => "Invalid parameter",
                    32774 => "System is in use",
                    32775 => "Invalid state for this operation",
                    32776 => "Incorrect data type",
                    32777 => "System is not available",
                    32778 => "Out of memory",
                    _ => "Unknown",
                };
                Err(wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint failed: {description} ({return_value})").into()))
            }
        }
    }

    pub(crate) async fn remove_associated_data(&self, affected_reference_point: VirtualSystemReferencePoint) -> wmi::WMIResult<JobState> {
        let remove_associated_data_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("RemoveAssociatedData")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("RemoveAssociatedData method signature not found".into()))?;
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT * FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let input = remove_associated_data_method_class.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", affected_reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;
        let result = self
            .connection
            .exec_method(&self.path, "RemoveAssociatedData", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("RemoveAssociatedData returned no output".into()))?
            .into_desr::<MethodResult>()?;
        match result.return_value {
            0 => {
                log::debug!("RemoveAssociatedData completed synchronously");
                return Ok(JobState::Completed);
            },
            4096 => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("RemoveAssociatedData returned no job".into()))?;
                log::debug!("Waiting for RemoveAssociatedData job: {path}");
                let job_state = Job::wait(&self.connection, path, &mut job_events).await?.job_state;
                log::debug!("RemoveAssociatedData job completed with state: {job_state:?}");
                return Ok(job_state);
            },
            return_value => {
                let description = match return_value {
                    32768 => "Failed",
                    32769 => "Access denied",
                    32770 => "Not supported",
                    32771 => "Status is unknown",
                    32772 => "Timeout",
                    32773 => "Invalid parameter",
                    32774 => "System is in use",
                    32775 => "Invalid state for this operation",
                    32776 => "Incorrect data type",
                    32777 => "System is not available",
                    32778 => "Out of memory",
                    _ => "Unknown",
                };
                Err(wmi::WMIError::ConvertVariantError(format!("RemoveAssociatedData failed: {description} ({return_value})").into()))
            }
        }
    }

    pub(crate) async fn import_metadata(&self, affected_system: &VirtualMachine, config_file_path: PathBuf, runtime_state_file_path: PathBuf) -> wmi::WMIResult<VirtualMachine> {
        unimplemented!()
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
            properties: vec![
                Property {
                    name: "ConsistencyLevel".into(),
                    cim_type: "uint8".into(),
                    value: (ConsistencyLevel::Crash as u8).to_string(),
                },
            ],
        }
    }

    async fn create_reference_point(service: &VirtualSystemReferencePointService, connection: &wmi::WMIConnection) -> VirtualSystemReferencePoint {
        log::info!("Creating reference point");
        let reference_point = service
            .create(&configured_vm(connection).await, Some(reference_point_settings()), ReferencePointType::Rct, None)
            .await
            .expect("reference point should be created");
        log::info!("Reference point created: {}", reference_point.path.as_str());
        log::info!("Waiting five seconds before the next reference-point operation");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        reference_point
    }

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM"]
    async fn creates_reference_point_for_configured_vm() {  // TODO: This always returns Not support
        let _ = env_logger::try_init();
        log::info!("Starting creates_reference_point_for_configured_vm");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
        log::info!("Connected to Hyper-V WMI namespace: {}", HYPER_V_NAMESPACE);
        let service = VirtualSystemReferencePointService::new(connection.clone()).expect("Hyper-V reference-point service should be available");
        log::info!("Resolved Hyper-V virtual system reference-point service");
        let reference_point = create_reference_point(&service, &connection).await;
        log::info!("Destroying reference point created by create test");
        let job_state = service.destroy(reference_point).await.expect("reference point should be destroyed");
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
        let job_state = service.destroy(reference_point).await.expect("reference point should be destroyed");
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
        let job_state = service
            .remove_associated_data(reference_point.clone())
            .await
            .expect("associated data should be removed");
        assert!(matches!(job_state, JobState::Completed));
        let job_state = service.destroy(reference_point).await.expect("reference point should be destroyed");
        assert!(matches!(job_state, JobState::Completed));
        log::info!("Finished removes_associated_data_for_configured_vm");
    }
}
