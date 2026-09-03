#![allow(non_snake_case)]

mod concretejob;
mod createreferencepointparams;
mod createreferencepointresult;
mod exportreferencepointinput;
mod importreferencepointmetadatainput;
mod methodresult;
mod referencepointinput;
mod virtualsystemreferencepointserviceclass;
mod virtualsystemreferencepointserviceinstance;

use self::concretejob::ConcreteJobModificationEvent;
use self::createreferencepointparams::CreateReferencePointParams;
use self::createreferencepointresult::CreateReferencePointResult;
use self::exportreferencepointinput::ExportReferencePointInput;
use self::importreferencepointmetadatainput::ImportReferencePointMetadataInput;
use self::methodresult::MethodResult;
use self::referencepointinput::ReferencePointInput;
use self::virtualsystemreferencepointserviceclass::VirtualSystemReferencePointServiceClass;
use self::virtualsystemreferencepointserviceinstance::VirtualSystemReferencePointServiceInstance;
use crate::model::ReferencePointId;
use crate::model::VmId;
use crate::service::referencepointservice::ReferencePointSettingsData;
use crate::wmi::HYPER_V_NAMESPACE;
use futures::StreamExt;
use serde::Deserialize;

pub struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    path: String,
}

const JOB_STATE_COMPLETED: u16 = 7;
const JOB_STATE_TERMINATED: u16 = 8;
const JOB_STATE_KILLED: u16 = 9;
const JOB_STATE_EXCEPTION: u16 = 10;

#[derive(Deserialize)]
#[serde(rename = "Msvm_ComputerSystem")]
struct ComputerSystemPath {
    __Path: Option<Option<String>>,
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
        &self, affected_system: &VmId, reference_point_settings: Option<&ReferencePointSettingsData>, reference_point_type: u16, resulting_reference_point: Option<ReferencePointId>,
    ) -> wmi::WMIResult<CreateReferencePointResult> {
        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;

        let resulting_reference_point = resulting_reference_point.map(|id| id.to_string());
        let affected_system_path = self
            .connection
            .async_raw_query::<ComputerSystemPath>(&format!("SELECT * FROM Msvm_ComputerSystem WHERE Name = '{}'", affected_system))
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Virtual machine not found: {affected_system}").into()))?
            .__Path;
        let affected_system_path = affected_system_path
            .and_then(|path| path)
            .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Virtual machine path not available: {affected_system}").into()))?;
        let reference_point_settings_xml = reference_point_settings.map(Self::reference_point_settings_to_xml);

        let result: CreateReferencePointResult = self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "CreateReferencePoint",
            CreateReferencePointParams {
                AffectedSystem: wmi::Variant::String(affected_system_path),
                ReferencePointSettings: reference_point_settings_xml.as_deref().unwrap_or_default(),
                ReferencePointType: reference_point_type,
                ResultingReferencePoint: resulting_reference_point.as_deref(),
            },
        )?;

        if let Some(job) = result.Job.as_ref().and_then(|job| job.as_deref()) {
            Self::wait_for_job(job, &mut job_events).await?;
        }

        Ok(result)
    }

    fn reference_point_settings_to_xml(settings: &ReferencePointSettingsData) -> String {
        let consistency_level = settings
            .consistency_level
            .map(|value| format!("<PROPERTY NAME=\"ConsistencyLevel\" TYPE=\"uint8\"><VALUE>{}</VALUE></PROPERTY>", value as u8))
            .unwrap_or_default();
        let caption = settings
            .caption
            .as_deref()
            .map(|value| format!("<PROPERTY NAME=\"Caption\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>"))
            .unwrap_or_default();
        let description = settings
            .description
            .as_deref()
            .map(|value| format!("<PROPERTY NAME=\"Description\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>"))
            .unwrap_or_default();
        let element_name = settings
            .element_name
            .as_deref()
            .map(|value| format!("<PROPERTY NAME=\"ElementName\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>"))
            .unwrap_or_default();

        format!("<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\">{consistency_level}{caption}{description}{element_name}</INSTANCE>")
    }

    async fn wait_for_job(job: &str, job_events: &mut (impl futures::Stream<Item = wmi::WMIResult<ConcreteJobModificationEvent>> + Unpin)) -> wmi::WMIResult<()> {
        while let Some(event) = job_events.next().await {
            let event = event?;
            let Some(path) = event.TargetInstance.__PATH.as_ref().and_then(|path| path.as_deref()) else {
                continue;
            };
            if !path.eq_ignore_ascii_case(job) {
                continue;
            }

            match event.TargetInstance.JobState {
                JOB_STATE_COMPLETED => return Ok(()),
                JOB_STATE_TERMINATED | JOB_STATE_KILLED | JOB_STATE_EXCEPTION => {
                    let description = event.TargetInstance.ErrorDescription.unwrap_or_else(|| "no error description".into());
                    let error_code = event.TargetInstance.ErrorCode.map(|code| format!(" ({code})")).unwrap_or_default();
                    return Err(wmi::WMIError::ConvertVariantError(format!("Concrete job failed{error_code}: {description}").into()));
                }
                _ => {}
            }
        }

        Err(wmi::WMIError::ConvertVariantError(format!("Concrete job event stream ended before completion: {job}").into()))
    }

    pub fn reference_point(result: CreateReferencePointResult) -> wmi::WMIResult<ReferencePointId> {
        let id = result.ResultingReferencePoint.as_ref().and_then(|id| id.as_deref())
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("CreateReferencePoint returned no reference point".into()))?;
        uuid::Uuid::parse_str(id).map_err(|error| wmi::WMIError::ConvertVariantError(format!("Invalid reference point identifier: {error}").into()))
    }

    pub async fn destroy_reference_point(&self, affected_reference_point: &str) -> wmi::WMIResult<MethodResult> {
        self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
            &self.path,
            "DestroyReferencePoint",
            ReferencePointInput {
                AffectedReferencePoint: affected_reference_point,
            },
        )
    }

    pub async fn export_reference_point(&self, reference_point: &str, export_directory: &str, export_setting_data: &str) -> wmi::WMIResult<MethodResult> {
        self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
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
        self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
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
        self.connection.exec_instance_method::<VirtualSystemReferencePointServiceClass, _>(
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
        let reference_point = VirtualSystemReferencePointService::reference_point(CreateReferencePointResult {
            ReturnValue: 0,
            ResultingReferencePoint: Some(Some("00000000-0000-0000-0000-000000000000".into())),
            Job: None,
        })
        .unwrap();

        assert_eq!(reference_point, uuid::Uuid::nil());
    }

    #[test]
    fn rejects_missing_created_reference_point() {
        let result = VirtualSystemReferencePointService::reference_point(CreateReferencePointResult {
            ReturnValue: 0,
            ResultingReferencePoint: None,
            Job: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn omits_empty_reference_point_settings_parameter() {
        let settings = ReferencePointSettingsData::new();

        assert!(
            None::<&ReferencePointSettingsData>
                .filter(|settings| settings.element_name.is_some() || settings.caption.is_some() || settings.description.is_some() || settings.consistency_level.is_some())
                .map(VirtualSystemReferencePointService::reference_point_settings_to_xml)
                .is_none()
        );
        assert!(
            Some(&settings)
                .filter(|settings| settings.element_name.is_some() || settings.caption.is_some() || settings.description.is_some() || settings.consistency_level.is_some())
                .map(VirtualSystemReferencePointService::reference_point_settings_to_xml)
                .is_none()
        );
    }

    #[test]
    fn serializes_only_populated_reference_point_settings() {
        let settings = ReferencePointSettingsData {
            element_name: Some("reference point".into()),
            caption: None,
            description: None,
            consistency_level: None,
        };

        assert_eq!(
            VirtualSystemReferencePointService::reference_point_settings_to_xml(&settings),
            "<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\"><PROPERTY NAME=\"ElementName\" TYPE=\"string\"><VALUE>reference point</VALUE></PROPERTY></INSTANCE>"
        );
    }
}
