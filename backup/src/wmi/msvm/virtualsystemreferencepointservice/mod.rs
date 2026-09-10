#![allow(non_snake_case)]

mod concretejob;
mod createreferencepointout;
mod exportreferencepointinput;
mod importreferencepointmetadatainput;
mod jobstate;
mod methodresult;
mod referencepointinput;
mod returnvalue;
mod virtualsystemmanagementserviceclass;
mod virtualsystemreferencepointserviceclass;
mod virtualsystemreferencepointserviceinstance;
mod virtualsystemsettingdata;

use self::concretejob::{ConcreteJobConfiguration, ConcreteJobModificationEvent};
use self::createreferencepointout::CreateReferencePointOut;
use self::exportreferencepointinput::ExportReferencePointInput;
use self::importreferencepointmetadatainput::ImportReferencePointMetadataInput;
use self::jobstate::JobState;
use self::methodresult::MethodResult;
use self::referencepointinput::ReferencePointInput;
use self::returnvalue::ReturnValue;
use self::virtualsystemmanagementserviceclass::VirtualSystemManagementServiceClass;
use self::virtualsystemreferencepointserviceclass::VirtualSystemReferencePointServiceClass;
use self::virtualsystemreferencepointserviceinstance::VirtualSystemReferencePointServiceInstance;
use self::virtualsystemsettingdata::VirtualSystemSettingData;
use crate::model::{ReferencePoint, ReferencePointId, VirtualMachine};
use crate::service::referencepointservice::{ConsistencyLevel, ReferencePointSettingData};
use crate::wmi::HYPER_V_NAMESPACE;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

pub struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    management_service_path: String,
    path: String,
}

impl VirtualSystemReferencePointService {
    pub async fn new() -> wmi::WMIResult<Self> {
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)?;
        let service = connection
            .async_raw_query::<VirtualSystemReferencePointServiceInstance>("SELECT * FROM Msvm_VirtualSystemReferencePointService")
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Reference point service not found".into()))?;
        let management_service = connection
            .async_raw_query::<VirtualSystemManagementServiceClass>("SELECT * FROM Msvm_VirtualSystemManagementService")
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system management service not found".into()))?;

        Ok(Self {
            connection,
            management_service_path: management_service.__Path,
            path: service.__Path,
        })
    }

    async fn ensure_incremental_backup_enabled(&self, affected_system: &VirtualMachine) -> wmi::WMIResult<()> {
        let settings_path = format!(
            "ASSOCIATORS OF {{{}}} WHERE AssocClass = Msvm_SettingsDefineState ResultClass = Msvm_VirtualSystemSettingData",
            Self::relative_wmi_path(affected_system.path().as_str()),
        );
        let settings = self
            .connection
            .async_raw_query::<VirtualSystemSettingData>(&settings_path)
            .await?
            .into_iter()
            .find(|settings| settings.is_current())
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Current virtual system settings not found".into()))?;

        log::info!("IncrementalBackupEnabled={}", settings.incremental_backup_enabled());
        if settings.incremental_backup_enabled() {
            return Ok(());
        }

        log::info!("Enabling incremental backup for virtual machine at {}", affected_system.path());
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let result: MethodResult = self
            .connection
            .exec_instance_method::<VirtualSystemManagementServiceClass, _>(&self.management_service_path, "ModifySystemSettings", settings.enable_incremental_backup_input())?;
        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("ModifySystemSettings failed with return value {return_value}").into()))?;
        log::info!("ModifySystemSettings returned with return value: {:?}", return_value);
        if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("ModifySystemSettings returned no job for asynchronous operation".into()))?;
            let job_id = self.get_job_instance(job).await?;
            match Self::wait_for_job(&job_id, &mut job_events).await? {
                JobState::Completed => Ok(()),
                state => Err(wmi::WMIError::ConvertVariantError(format!("ModifySystemSettings job {job_id} returned unexpected state: {state:?}").into())),
            }
        } else if return_value == ReturnValue::Completed {
            Ok(())
        } else {
            Err(wmi::WMIError::ConvertVariantError(format!("ModifySystemSettings returned unexpected value {return_value:?}").into()))
        }
    }

    pub async fn create_reference_point(
        &self, affected_system: &VirtualMachine, reference_point_settings: Option<&ReferencePointSettingData>, reference_point_type: u16, resulting_reference_point: Option<ReferencePointId>,
    ) -> wmi::WMIResult<ReferencePoint> {
        log::info!("Creating reference point for virtual machine at {}", affected_system.path());
        self.ensure_incremental_backup_enabled(affected_system).await?;

        let reference_point_settings_xml = reference_point_settings
            .map(ReferencePointSettingData::to_xml)
            .unwrap_or_else(|| ReferencePointSettingData::new(ConsistencyLevel::Crash).to_xml());
        
        let resulting_reference_point = resulting_reference_point.map(|id| id.to_string());
        let affected_system = affected_system.path().as_str().to_owned();

        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;

        let method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("CreateReferencePoint")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint method signature not found".into()))?;
        let input = method_class.spawn_instance()?;
        input
            .put_property("AffectedSystem", affected_system)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedSystem: {error}").into()))?;
        input
            .put_property("ReferencePointSettings", reference_point_settings_xml)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointSettings: {error}").into()))?;
        input
            .put_property("ReferencePointType", reference_point_type)
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ReferencePointType: {error}").into()))?;
        if let Some(reference_point) = resulting_reference_point {
            input
                .put_property("ResultingReferencePoint", reference_point)
                .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set ResultingReferencePoint: {error}").into()))?;
        }
        let result = self
            .connection
            .exec_method(&self.path, "CreateReferencePoint", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no output".into()))?
            .into_desr::<CreateReferencePointOut>()?;
        log::debug!(
            "CreateReferencePoint returned value {} with job {:?} and resulting reference point {:?}",
            result.ReturnValue,
            result.Job,
            result.ResultingReferencePoint
        );

        let return_value = ReturnValue::try_from(result.ReturnValue as u16).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint failed with return value {return_value}").into()))?;
        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no job for asynchronous operation".into()))?;
            let job: String = self.get_job_instance(job).await?;
            log::info!("Waiting for CreateReferencePoint job {job}");

            let job_state = Self::wait_for_job(&job, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => ReturnValue::Completed,
                Ok(state) => {
                    log::error!("CreateReferencePoint job {job} returned unexpected state: {state:?}");
                    return Err(wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint job {job} returned unexpected state: {state:?}").into()));
                }
                Err(error) => {
                    log::error!("CreateReferencePoint job {job} failed: {error}");
                    return Err(error);
                }
            }
        } else {
            return_value
        };

        log::debug!("CreateReferencePoint completed with return value {return_value:?}");

        let job_path = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no job".into()))?;
        let reference_point = self.get_reference_point_for_job(job_path).await?;
        ReferencePoint::try_from(reference_point).map_err(|error| wmi::WMIError::ConvertVariantError(error.to_string().into()))
    }

    async fn wait_for_job(job_id: &str, job_events: &mut (impl futures::Stream<Item = wmi::WMIResult<ConcreteJobModificationEvent>> + Unpin)) -> wmi::WMIResult<JobState> {
        log::debug!("Waiting for concrete job {job_id}");
        timeout(Duration::from_secs(1800), async {
            while let Some(result) = job_events.next().await {
                let event = result?;
                let instance_id = event.TargetInstance.InstanceID;
                if instance_id != job_id {
                    log::trace!("Ignoring concrete job event for instance ID {instance_id:?} while waiting for {job_id}");
                    continue;
                }

                match JobState::try_from(event.TargetInstance.JobState) {
                    Ok(JobState::Completed) => {
                        log::info!("Concrete job {job_id} completed");
                        return Ok(JobState::Completed);
                    }
                    Err(JobState::Terminated | JobState::Killed | JobState::Exception) => {
                        let description = event.TargetInstance.ErrorDescription.unwrap_or_else(|| "no error description".into());
                        let error_code = event.TargetInstance.ErrorCode;
                        log::error!("Concrete job {job_id} failed {error_code}: {description}");
                        return Err(wmi::WMIError::ConvertVariantError(format!("Concrete job failed {error_code}: {description}").into()));
                    }
                    state => {
                        log::trace!("Concrete job {job_id} reported nonterminal state {state:?}; waiting for next event");
                    }
                }
            }

            Err(wmi::WMIError::ConvertVariantError(format!("Concrete job event stream ended before completion: {job_id}").into()))
        })
        .await
        .map_err(|_| {
            log::error!("Timed out waiting for concrete job: {job_id}");
            wmi::WMIError::ConvertVariantError(format!("Timed out waiting for concrete job: {job_id}").into())
        })?
    }

    async fn get_reference_point_for_job(&self, job_path: &str) -> wmi::WMIResult<crate::wmi::msvm::referencepoint::ReferencePoint> {
        let relative_job_path = Self::relative_wmi_path(job_path);
        let mut reference_points = self
            .connection
            .async_raw_query::<crate::wmi::msvm::referencepoint::ReferencePoint>(&format!("ASSOCIATORS OF {{{relative_job_path}}} WHERE AssocClass = CIM_AffectedJobElement ResultClass = Msvm_VirtualSystemReferencePoint"))
            .await?;
        let reference_point = reference_points.pop().ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Reference point not found for job: {job_path}").into()))?;
        Ok(reference_point)
    }

    async fn get_job_instance(&self, job_path: &str) -> wmi::WMIResult<String> {
        let relative_job_path = Self::relative_wmi_path(job_path);
        self.connection.get_by_path::<ConcreteJobConfiguration>(relative_job_path).map(|job| job.InstanceID)
    }

    fn relative_wmi_path(path: &str) -> &str {
        path.rsplit_once(':').map_or(path, |(_, relative_path)| relative_path)
    }

    pub async fn destroy_reference_point(&self, affected_reference_point: &str) -> wmi::WMIResult<ReturnValue> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let result: MethodResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "DestroyReferencePoint",
            ReferencePointInput {
                AffectedReferencePoint: affected_reference_point,
            },
        )?;

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint failed with return value {return_value}").into()))?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no job for asynchronous operation".into()))?;
            let job_id = self.get_job_instance(job).await?;
            let job_state = Self::wait_for_job(&job_id, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => ReturnValue::Completed,
                Ok(state) => {
                    log::error!("DestroyReferencePoint job {job_id} returned unexpected state: {state:?}");
                    return Err(wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint job {job_id} returned unexpected state: {state:?}").into()));
                }
                Err(error) => {
                    log::error!("DestroyReferencePoint job {job_id} failed: {error}");
                    return Err(error);
                }
            }
        } else {
            return_value
        };

        log::debug!("DestroyReferencePoint completed with return value {return_value:?}");
        Ok(return_value)
    }

    #[allow(dead_code)]
    pub async fn export_reference_point(&self, reference_point: &str, export_directory: &str, export_setting_data: &str) -> wmi::WMIResult<ReturnValue> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let result: MethodResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "ExportReferencePoint",
            ExportReferencePointInput {
                ReferencePoint: reference_point,
                ExportDirectory: export_directory,
                ExportSettingData: export_setting_data,
            },
        )?;

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("ExportReferencePoint failed with return value {return_value}").into()))?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportReferencePoint returned no job for asynchronous operation".into()))?;
            let job_id = self.get_job_instance(job).await?;
            let job_state = Self::wait_for_job(&job_id, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => ReturnValue::Completed,
                Ok(state) => {
                    log::error!("ExportReferencePoint job {job_id} returned unexpected state: {state:?}");
                    return Err(wmi::WMIError::ConvertVariantError(format!("ExportReferencePoint job {job_id} returned unexpected state: {state:?}").into()));
                }
                Err(error) => {
                    log::error!("ExportReferencePoint job {job_id} failed: {error}");
                    return Err(error);
                }
            }
        } else {
            return_value
        };

        log::debug!("ExportReferencePoint completed with return value {return_value:?}");
        Ok(return_value)
    }

    #[allow(dead_code)]
    pub async fn import_reference_point_metadata(&self, affected_system: &str, config_file_path: &str, runtime_state_file_path: &str) -> wmi::WMIResult<ReturnValue> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let result: MethodResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "ImportReferencePointMetadata",
            ImportReferencePointMetadataInput {
                AffectedSystem: affected_system,
                ConfigFilePath: config_file_path,
                RuntimeStateFilePath: runtime_state_file_path,
            },
        )?;

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("ImportReferencePointMetadata failed with return value {return_value}").into()))?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("ImportReferencePointMetadata returned no job for asynchronous operation".into()))?;
            let job_id = self.get_job_instance(job).await?;
            let job_state = Self::wait_for_job(&job_id, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => ReturnValue::Completed,
                Ok(state) => {
                    log::error!("ImportReferencePointMetadata job {job_id} returned unexpected state: {state:?}");
                    return Err(wmi::WMIError::ConvertVariantError(format!("ImportReferencePointMetadata job {job_id} returned unexpected state: {state:?}").into()));
                }
                Err(error) => {
                    log::error!("ImportReferencePointMetadata job {job_id} failed: {error}");
                    return Err(error);
                }
            }
        } else {
            return_value
        };

        log::debug!("ImportReferencePointMetadata completed with return value {return_value:?}");
        Ok(return_value)
    }

    pub async fn remove_associated_data(&self, affected_reference_point: &str) -> wmi::WMIResult<ReturnValue> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;
        let result: MethodResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "RemoveAssociatedData",
            ReferencePointInput {
                AffectedReferencePoint: affected_reference_point,
            },
        )?;

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| wmi::WMIError::ConvertVariantError(format!("RemoveAssociatedData failed with return value {return_value}").into()))?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result
                .Job
                .as_deref()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("RemoveAssociatedData returned no job for asynchronous operation".into()))?;
            let job_id = self.get_job_instance(job).await?;
            let job_state = Self::wait_for_job(&job_id, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => ReturnValue::Completed,
                Ok(state) => {
                    log::error!("RemoveAssociatedData job {job_id} returned unexpected state: {state:?}");
                    return Err(wmi::WMIError::ConvertVariantError(format!("RemoveAssociatedData job {job_id} returned unexpected state: {state:?}").into()));
                }
                Err(error) => {
                    log::error!("RemoveAssociatedData job {job_id} failed: {error}");
                    return Err(error);
                }
            }
        } else {
            return_value
        };

        log::debug!("RemoveAssociatedData completed with return value {return_value:?}");
        Ok(return_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::referencepointservice::ConsistencyLevel;

    #[test]
    fn serializes_default_reference_point_settings() {
        let settings = ReferencePointSettingData::new(ConsistencyLevel::Crash);

        assert_eq!(
            settings.to_xml(),
            "<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\"><PROPERTY NAME=\"ConsistencyLevel\" TYPE=\"uint8\"><VALUE>0</VALUE></PROPERTY></INSTANCE>"
        );
    }

    #[test]
    fn serializes_only_populated_reference_point_settings() {
        let settings = ReferencePointSettingData {
            caption: None,
            element_name: Some("reference point".into()),
            consistency_level: ConsistencyLevel::Application,
        };

        assert_eq!(
            settings.to_xml(),
            "<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\"><PROPERTY NAME=\"ConsistencyLevel\" TYPE=\"uint8\"><VALUE>1</VALUE></PROPERTY><PROPERTY NAME=\"ElementName\" TYPE=\"string\"><VALUE>reference point</VALUE></PROPERTY></INSTANCE>"
        );
    }
}
