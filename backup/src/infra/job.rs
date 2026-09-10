use std::time::Duration;
use futures::StreamExt;

use super::*;

#[derive(Debug)]
pub(super) struct Job {
    pub(super) connection: wmi::WMIConnection,
    pub(super) path: String,
    pub(super) instance_id: InstanceId,
    pub(super) name: String,
    pub(super) state: JobState,
    pub(super) last_modified: chrono::DateTime<chrono::Utc>,
    pub(super) removal: Duration,
}

impl Job {
    pub(super) async fn wait(
        connection: &wmi::WMIConnection,
        path: String,
        job_events: &mut (impl futures::Stream<Item = wmi::WMIResult<ConcreteJobModificationEvent>> + Unpin),
    ) -> wmi::WMIResult<Self> {
        let job_id = connection.get_by_path::<JobOut>(&path)?.instance_id().to_owned();
        while let Some(result) = job_events.next().await {
            let event = result?;
            let concrete_job = event.TargetInstance;
            if concrete_job.instance_id() != job_id {
                log::trace!("Ignoring concrete job event for instance ID {} while waiting for {}", concrete_job.instance_id(), job_id);
                continue;
            }
            match concrete_job.JobState {
                2 => log::trace!("Job state is new"),
                3 => log::trace!("Job state is starting"),
                4 => log::trace!("Job state is running"),
                5 => return Err(wmi::WMIError::ConvertVariantError("The Job is suspended, and can be restarted in a seamless manner".into())), // TODO How?
                6 => log::trace!("Job state is shutting down"),
                7 => return Self::from_concrete(concrete_job, path.clone(), connection.clone()),
                8 => return Err(wmi::WMIError::ConvertVariantError("The job has been terminated".into())),  // Terminated
                9 => return Err(wmi::WMIError::ConvertVariantError("The job has been killed".into())),  // Killed
                10 => return Err(wmi::WMIError::ConvertVariantError("The job is in an exception state".into())),  // Exception
                11 => return Err(wmi::WMIError::ConvertVariantError("The job is in a vendor-specific state that supports problem discovery, or resolution, or both".into())),  // Service lost
                12 => return Err(wmi::WMIError::ConvertVariantError("The job is in a pending query state".into())),  // TODO: How?
                13..=32767 => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in a DMTF reserved state ({})", concrete_job.JobState).into())),
                32768..=65535 => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in a vendor reserved state ({})", concrete_job.JobState).into())),
                _ => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in an unknown state ({})", concrete_job.JobState).into())),
            }
        }
        Err(wmi::WMIError::ConvertVariantError(format!("Concrete job event stream ended before completion: {job_id}").into()))
    }

    pub(super) fn from_concrete(job: JobOut, path: String, connection: wmi::WMIConnection) -> Result<Self, wmi::WMIError> {
        Ok(Self {
            connection,
            path,
            instance_id: job.InstanceID.into(),
            name: job.Name,
            state: JobState::try_from(job.JobState).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            last_modified: job.TimeOfLastStateChange.0.with_timezone(&chrono::Utc),
            removal: job.TimeBeforeRemoval.0,
        })
    }

    pub async fn get_related<T>(&self, relation: &str) -> Result<T, wmi::WMIError>
    where
        T: serde::de::DeserializeOwned,
    {
        let job_path = self.path.as_str();
        let query = format!("ASSOCIATORS OF {{{job_path}}} WHERE AssocClass = CIM_AffectedJobElement ResultClass = {relation}");

        let related_objects = self.connection.raw_query::<std::collections::HashMap<String, serde_json::Value>>(&query)?;
        let related_object = related_objects.into_iter().next().ok_or_else(|| {
            wmi::WMIError::ConvertVariantError(format!("Related object not found for job: {job_path}").into())
        })?;
        let related_object = if related_object.contains_key("__Path") {
            related_object
        } else {
            let instance_id = related_object
                .get("InstanceID")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| wmi::WMIError::ConvertVariantError("Related object has no InstanceID".into()))?;
            let query = format!("SELECT * FROM {relation} WHERE InstanceID = '{instance_id}'");
            let mut related_object = self.connection
                .raw_query::<std::collections::HashMap<String, serde_json::Value>>(&query)?
                .into_iter()
                .next()
                .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Related object {instance_id} not found").into()))?
            ;
            let authority = job_path.split_once(':').map(|(authority, _)| authority).unwrap_or_default();
            related_object.insert(
                "__Path".to_owned(),
                serde_json::Value::String(format!(r#"{authority}:{relation}.InstanceID="{instance_id}""#)),
            );
            related_object
        };
        serde_json::from_value(serde_json::Value::Object(related_object.into_iter().collect())).map_err(|error| {
            wmi::WMIError::ConvertVariantError(format!("Failed to deserialize related object: {error}").into())
        })
    }
}