#![allow(non_snake_case)]

mod concretejob;
mod createreferencepointinput;
mod createreferencepointresult;
mod exportreferencepointinput;
mod importreferencepointmetadatainput;
mod methodresult;
mod referencepointinput;
mod virtualsystemreferencepointserviceclass;
mod virtualsystemreferencepointserviceinstance;

use self::concretejob::ConcreteJobModificationEvent;
use self::createreferencepointinput::CreateReferencePointInput;
use self::createreferencepointresult::CreateReferencePointResult;
use self::exportreferencepointinput::ExportReferencePointInput;
use self::importreferencepointmetadatainput::ImportReferencePointMetadataInput;
use self::methodresult::MethodResult;
use self::referencepointinput::ReferencePointInput;
use self::virtualsystemreferencepointserviceclass::VirtualSystemReferencePointServiceClass;
use self::virtualsystemreferencepointserviceinstance::VirtualSystemReferencePointServiceInstance;
use crate::model::ReferencePoint;
use crate::wmi::HYPER_V_NAMESPACE;
use futures::StreamExt;

pub struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    path: String,
}

const JOB_STATE_COMPLETED: u16 = 7;
const JOB_STATE_TERMINATED: u16 = 8;
const JOB_STATE_KILLED: u16 = 9;
const JOB_STATE_EXCEPTION: u16 = 10;

impl VirtualSystemReferencePointService {
    pub async fn new() -> wmi::WMIResult<Self> {
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)?;
        let service = connection
            .async_raw_query::<VirtualSystemReferencePointServiceInstance>(
                "SELECT __Path FROM Msvm_VirtualSystemReferencePointService",
            )
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| {
                wmi::WMIError::ConvertVariantError("Reference point service not found".into())
            })?;

        Ok(Self {
            connection,
            path: service.__Path,
        })
    }

    pub async fn create_reference_point(&self, affected_system: &str, reference_point_settings: &str, reference_point_type: u16, resulting_reference_point: &str) -> wmi::WMIResult<CreateReferencePointResult> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>(
                "SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'",
            )?;
        let result: CreateReferencePointResult = self
            .connection
            .exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "CreateReferencePoint",
            CreateReferencePointInput {
                AffectedSystem: affected_system,
                ReferencePointSettings: reference_point_settings,
                ReferencePointType: reference_point_type,
                ResultingReferencePoint: resulting_reference_point,
            },
        )?;

        if let Some(job) = result.Job.as_deref() {
            Self::wait_for_job(job, &mut job_events).await?;
        }

        Ok(result)
    }

    async fn wait_for_job(job: &str, job_events: &mut (impl futures::Stream<Item = wmi::WMIResult<ConcreteJobModificationEvent>> + Unpin)) -> wmi::WMIResult<()> {
        while let Some(event) = job_events.next().await {
            let event = event?;
            if !event.TargetInstance.__PATH.eq_ignore_ascii_case(job) {
                continue;
            }

            match event.TargetInstance.JobState {
                JOB_STATE_COMPLETED => return Ok(()),
                JOB_STATE_TERMINATED | JOB_STATE_KILLED | JOB_STATE_EXCEPTION => {
                    let description = event
                        .TargetInstance
                        .ErrorDescription
                        .unwrap_or_else(|| "no error description".into());
                    let error_code = event
                        .TargetInstance
                        .ErrorCode
                        .map(|code| format!(" ({code})"))
                        .unwrap_or_default();
                    return Err(wmi::WMIError::ConvertVariantError(
                        format!("Concrete job failed{error_code}: {description}").into(),
                    ));
                }
                _ => {}
            }
        }

        Err(wmi::WMIError::ConvertVariantError(
            format!("Concrete job event stream ended before completion: {job}").into(),
        ))
    }

    pub fn reference_point(result: CreateReferencePointResult) -> wmi::WMIResult<ReferencePoint> {
        result
            .ResultingReferencePoint
            .map(|id| {
                uuid::Uuid::parse_str(&id)
                    .map(ReferencePoint::new)
                    .map_err(|error| {
                        wmi::WMIError::ConvertVariantError(
                            format!("Invalid reference point identifier: {error}").into(),
                        )
                    })
            })
            .transpose()?
            .ok_or_else(|| {
                wmi::WMIError::ConvertVariantError(
                    "CreateReferencePoint returned no reference point".into(),
                )
            })
    }

    pub async fn destroy_reference_point(&self, affected_reference_point: &str) -> wmi::WMIResult<MethodResult> {
        self.connection
            .exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
                &self.path,
                "DestroyReferencePoint",
                ReferencePointInput {
                    AffectedReferencePoint: affected_reference_point,
                },
            )
    }

    pub async fn export_reference_point(&self, reference_point: &str, export_directory: &str, export_setting_data: &str) -> wmi::WMIResult<MethodResult> {
        self.connection
            .exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
                &self.path,
                "ExportReferencePoint",
                ExportReferencePointInput {
                    ReferencePoint: reference_point,
                    ExportDirectory: export_directory,
                    ExportSettingData: export_setting_data,
                },
            )
    }

    pub async fn import_reference_point_metadata(&self, affected_system: &str, config_file_path: &str, runtime_state_file_path: &str) -> wmi::WMIResult<MethodResult> {
        self.connection
            .exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
                &self.path,
                "ImportReferencePointMetadata",
                ImportReferencePointMetadataInput {
                    AffectedSystem: affected_system,
                    ConfigFilePath: config_file_path,
                    RuntimeStateFilePath: runtime_state_file_path,
                },
            )
    }

    pub async fn remove_associated_data(&self, affected_reference_point: &str) -> wmi::WMIResult<MethodResult> {
        self.connection
            .exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
                &self.path,
                "RemoveAssociatedData",
                ReferencePointInput {
                    AffectedReferencePoint: affected_reference_point,
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_created_reference_point() {
        let reference_point =
            VirtualSystemReferencePointService::reference_point(CreateReferencePointResult {
                ReturnValue: 0,
                ResultingReferencePoint: Some("00000000-0000-0000-0000-000000000000".into()),
                Job: None,
            })
            .unwrap();

        assert_eq!(reference_point.id(), uuid::Uuid::nil());
    }

    #[test]
    fn rejects_missing_created_reference_point() {
        let result =
            VirtualSystemReferencePointService::reference_point(CreateReferencePointResult {
                ReturnValue: 0,
                ResultingReferencePoint: None,
                Job: None,
            });

        assert!(result.is_err());
    }
}
