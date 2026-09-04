#![allow(non_snake_case)]

mod concretejob;
mod createreferencepointparams;
mod createreferencepointresult;
mod exportreferencepointinput;
mod importreferencepointmetadatainput;
mod jobstate;
mod methodresult;
mod referencepointinput;
mod returnvalue;
mod virtualsystemreferencepointserviceclass;
mod virtualsystemreferencepointserviceinstance;

use self::concretejob::{ConcreteJobConfiguration, ConcreteJobModificationEvent};
use self::createreferencepointparams::CreateReferencePointParams;
use self::createreferencepointresult::CreateReferencePointResult;
use self::exportreferencepointinput::ExportReferencePointInput;
use self::importreferencepointmetadatainput::ImportReferencePointMetadataInput;
use self::jobstate::JobState;
use self::methodresult::MethodResult;
use self::referencepointinput::ReferencePointInput;
use self::returnvalue::ReturnValue;
use self::virtualsystemreferencepointserviceclass::VirtualSystemReferencePointServiceClass;
use self::virtualsystemreferencepointserviceinstance::VirtualSystemReferencePointServiceInstance;
use crate::model::{ReferencePoint, ReferencePointId, VirtualMachine};
use crate::service::referencepointservice::ReferencePointSettingData;
use crate::wmi::HYPER_V_NAMESPACE;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

pub struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
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

        Ok(Self { connection, path: service.__Path })
    }

    pub async fn create_reference_point(
        &self, affected_system: &VirtualMachine, reference_point_settings: Option<&ReferencePointSettingData>, reference_point_type: u16, resulting_reference_point: Option<ReferencePointId>,
    ) -> wmi::WMIResult<ReferencePoint> {
        log::info!("Creating reference point for virtual machine at {}", affected_system.path());
        let resulting_reference_point = resulting_reference_point.map(|id| id.to_string());
        let reference_point_settings_xml = reference_point_settings.map(ReferencePointSettingData::to_xml);
        
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;

        let result: CreateReferencePointResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "CreateReferencePoint",
            CreateReferencePointParams {
                AffectedSystem: wmi::Variant::String(affected_system.path().as_str().to_owned()),
                ReferencePointSettings: reference_point_settings_xml.as_deref().unwrap_or_default(),
                ReferencePointType: reference_point_type,
                ResultingReferencePoint: resulting_reference_point.as_deref(),
            },
        )?;
        log::debug!(
            "CreateReferencePoint returned value {} with job {:?} and resulting reference point {:?}",
            result.ReturnValue,
            result.Job,
            result.ResultingReferencePoint
        );

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| {
            wmi::WMIError::ConvertVariantError(format!("CreateReferencePoint failed with return value {return_value}").into())
        })?;
        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no job for asynchronous operation".into()))?;
            let job: String = self.get_job_instance(job).await?;
            log::info!("Waiting for CreateReferencePoint job {job}");

            let job_state = Self::wait_for_job(&job, &mut job_events).await;
            match job_state {
                Ok(JobState::Completed) => {
                    ReturnValue::Completed
                }
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
        let mut reference_points = self.connection
            .async_raw_query::<crate::wmi::msvm::referencepoint::ReferencePoint>(&format!("ASSOCIATORS OF {{{relative_job_path}}} WHERE AssocClass = CIM_AffectedJobElement ResultClass = Msvm_VirtualSystemReferencePoint"))
            .await?;
        let reference_point = reference_points.pop().ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Reference point not found for job: {job_path}").into()))?;
        Ok(reference_point)
    }

    async fn get_job_instance(&self, job_path: &str) -> wmi::WMIResult<String> {
        let relative_job_path = Self::relative_wmi_path(job_path);
        self.connection
            .get_by_path::<ConcreteJobConfiguration>(relative_job_path)
            .map(|job| job.InstanceID)
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

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| {
            wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint failed with return value {return_value}").into())
        })?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no job for asynchronous operation".into()))?;
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

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| {
            wmi::WMIError::ConvertVariantError(format!("ExportReferencePoint failed with return value {return_value}").into())
        })?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("ExportReferencePoint returned no job for asynchronous operation".into()))?;
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

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| {
            wmi::WMIError::ConvertVariantError(format!("ImportReferencePointMetadata failed with return value {return_value}").into())
        })?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("ImportReferencePointMetadata returned no job for asynchronous operation".into()))?;
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

        let return_value = ReturnValue::try_from(result.ReturnValue).map_err(|return_value| {
            wmi::WMIError::ConvertVariantError(format!("RemoveAssociatedData failed with return value {return_value}").into())
        })?;

        let return_value = if return_value == ReturnValue::MethodParametersCheckedAndJobStarted {
            let job = result.Job.as_deref().ok_or_else(|| wmi::WMIError::ConvertVariantError("RemoveAssociatedData returned no job for asynchronous operation".into()))?;
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
