use std::time::Duration;
use futures::StreamExt;

use super::*;

#[derive(Debug)]
pub(super) struct Job {
    pub(super) cancellable: bool,
    pub(super) caption: String,
    pub(super) communication_status: Option<CommunicationStatus>,
    pub(super) connection: wmi::WMIConnection,
    pub(super) delete_on_completion: bool,
    pub(super) description: String,
    pub(super) detailed_status: Option<DetailedStatus>,
    pub(super) elapsed_time: Duration,
    pub(super) element_name: String,
    pub(super) error_code: u16,
    pub(super) error_description: String,
    pub(super) error_summary_description: String,
    pub(super) health_state: HealthState,
    pub(super) install_date: chrono::DateTime<chrono::Utc>,
    pub(super) instance_id: InstanceId,
    pub(super) job_run_times: u32,
    pub(crate) job_state: JobState,
    pub(super) job_status: String,
    pub(super) job_type: JobType,
    pub(super) local_or_utc_time: LocalOrUtcTime,
    pub(super) name: String,
    pub(super) notify: String,
    pub(super) operating_status: Option<OperatingStatus>,
    pub(super) operational_status: Vec<OperationalStatus>,
    pub(super) other_recovery_action: String,
    pub(super) owner: String,
    pub(super) path: String,
    pub(super) percent_complete: Percent,
    pub(super) priority: u32,
    pub(super) primary_status: Option<PrimaryStatus>,
    pub(super) recovery_action: RecoveryAction,
    pub(super) run_month: Option<RunMonth>,
    pub(super) run_day: Option<RunDay>,
    pub(super) run_day_of_week: Option<RunDayOfWeek>,
    pub(super) run_start_interval: Option<Duration>,
    pub(super) scheduled_start_time: chrono::DateTime<chrono::Utc>,
    pub(super) start_time: chrono::DateTime<chrono::Utc>,
    pub(super) status: String,
    pub(super) status_descriptions: Vec<String>,
    pub(super) time_of_last_state_change: chrono::DateTime<chrono::Utc>,
    pub(super) time_submitted: chrono::DateTime<chrono::Utc>,
    pub(super) time_before_removal: Duration,
    pub(super) until_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for Job {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Job")
            .field("cancellable", &self.cancellable)
            .field("caption", &self.caption)
            .field("communication_status", &self.communication_status)
            .field("delete_on_completion", &self.delete_on_completion)
            .field("description", &self.description)
            .field("detailed_status", &self.detailed_status)
            .field("elapsed_time", &self.elapsed_time)
            .field("element_name", &self.element_name)
            .field("error_code", &self.error_code)
            .field("error_description", &self.error_description)
            .field("error_summary_description", &self.error_summary_description)
            .field("health_state", &self.health_state)
            .field("install_date", &self.install_date)
            .field("instance_id", &self.instance_id.as_str())
            .field("job_run_times", &self.job_run_times)
            .field("job_state", &self.job_state)
            .field("job_status", &self.job_status)
            .field("job_type", &self.job_type)
            .field("local_or_utc_time", &self.local_or_utc_time)
            .field("name", &self.name)
            .field("notify", &self.notify)
            .field("operating_status", &self.operating_status)
            .field("operational_status", &self.operational_status)
            .field("owner", &self.owner)
            .field("other_recovery_action", &self.other_recovery_action)
            .field("path", &self.path)
            .field("percent_complete", &self.percent_complete)
            .field("priority", &self.priority)
            .field("primary_status", &self.primary_status)
            .field("recovery_action", &self.recovery_action)
            .field("run_month", &self.run_month)
            .field("run_day", &self.run_day)
            .field("run_day_of_week", &self.run_day_of_week)
            .field("run_start_interval", &self.run_start_interval)
            .field("scheduled_start_time", &self.scheduled_start_time)
            .field("start_time", &self.start_time)
            .field("status", &self.status)
            .field("status_descriptions", &self.status_descriptions)
            .field("time_of_last_state_change", &self.time_of_last_state_change)
            .field("time_submitted", &self.time_submitted)
            .field("time_before_removal", &self.time_before_removal)
            .field("until_time", &self.until_time)
            .finish()
    }
}

impl Job {
    pub(super) async fn wait(
        connection: &wmi::WMIConnection,
        path: String,
        job_events: &mut (impl futures::Stream<Item = wmi::WMIResult<ConcreteJobModificationEvent>> + Unpin),
    ) -> wmi::WMIResult<Self> {
        let job_id = path
            .split_once("InstanceID=\"")
            .and_then(|(_, value)| value.split_once('"').map(|(instance_id, _)| instance_id.to_owned()))
            .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Concrete job path has no InstanceID: {path}").into()))?;
        while let Some(result) = job_events.next().await {
            let event = result.map_err(|error| {
                wmi::WMIError::ConvertVariantError(format!("Concrete job notification failed: {error}").into())
            })?;
            let concrete_job = event.TargetInstance;
            if concrete_job.InstanceID != job_id {
                log::trace!("Ignoring concrete job event for instance ID {} while waiting for {}", concrete_job.InstanceID, job_id);
                continue;
            }
            match concrete_job.JobState {
                2 => log::trace!("Job state is new"),
                3 => log::trace!("Job state is starting"),
                4 => log::trace!("Job state is running {}%", concrete_job.PercentComplete),
                5 => return Err(wmi::WMIError::ConvertVariantError("The Job is suspended, and can be restarted in a seamless manner".into())), // TODO How?
                6 => log::trace!("Job state is shutting down"),
                7 => {
                    log::trace!("Job state is completed");
                    return Self::from(concrete_job, path.clone(), connection.clone());
                },
                8 => return Err(wmi::WMIError::ConvertVariantError(format!("The job has been terminated ({})", concrete_job.JobState).into())), // Terminated
                9 => return Err(wmi::WMIError::ConvertVariantError(format!("The job has been killed ({})", concrete_job.JobState).into())), // Killed
                10 => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in an exception state ({}): {} ({})", concrete_job.JobState, concrete_job.ErrorDescription, concrete_job.ErrorCode).into())), // Exception
                11 => return Err(wmi::WMIError::ConvertVariantError("The job is in a vendor-specific state that supports problem discovery, or resolution, or both".into())), // Service lost
                12 => return Err(wmi::WMIError::ConvertVariantError("The job is in a pending query state".into())), // TODO: How?
                13..=32767 => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in a DMTF reserved state ({})", concrete_job.JobState).into())),
                32768..=65535 => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in a vendor reserved state ({})", concrete_job.JobState).into())),
                _ => return Err(wmi::WMIError::ConvertVariantError(format!("The job is in an unknown state ({})", concrete_job.JobState).into())),
            }
        }
        Err(wmi::WMIError::ConvertVariantError(format!("Concrete job event stream ended before completion: {job_id}").into()))
    }

    pub(super) fn from(job: JobOut, path: String, connection: wmi::WMIConnection) -> Result<Self, wmi::WMIError> {
        Ok(Self {
            cancellable: job.Cancellable,
            caption: job.Caption,
            communication_status: job.CommunicationStatus.map(CommunicationStatus::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            connection,
            delete_on_completion: job.DeleteOnCompletion,
            description: job.Description,
            detailed_status: job.DetailedStatus.map(DetailedStatus::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            elapsed_time: job.ElapsedTime.0,
            element_name: job.ElementName,
            error_code: job.ErrorCode,
            error_description: job.ErrorDescription,
            error_summary_description: job.ErrorSummaryDescription,
            health_state: HealthState::try_from(job.HealthState).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            install_date: job.InstallDate.0.with_timezone(&chrono::Utc),
            path,
            instance_id: job.InstanceID.into(),
            job_run_times: job.JobRunTimes,
            job_state: JobState::try_from(job.JobState).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            job_status: job.JobStatus,
            job_type: JobType::try_from(job.JobType).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            local_or_utc_time: LocalOrUtcTime::try_from(job.LocalOrUtcTime).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            name: job.Name,
            notify: job.Notify,
            operating_status: job.OperatingStatus.map(OperatingStatus::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            operational_status: job.OperationalStatus.into_iter().map(OperationalStatus::try_from).collect::<Result<_, _>>().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            other_recovery_action: job.OtherRecoveryAction,
            owner: job.Owner,
            percent_complete: Percent::try_from(job.PercentComplete).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            priority: job.Priority,
            primary_status: job.PrimaryStatus.map(PrimaryStatus::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            recovery_action: RecoveryAction::try_from(job.RecoveryAction).map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            run_day: job.RunDay.map(RunDay::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            run_day_of_week: job.RunDayOfWeek.map(RunDayOfWeek::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            run_month: job.RunMonth.map(RunMonth::try_from).transpose().map_err(|error| wmi::WMIError::ConvertVariantError(error.into()))?,
            run_start_interval: job.RunStartInterval.map(|value| value.0),
            scheduled_start_time: job.ScheduledStartTime.0.with_timezone(&chrono::Utc),
            start_time: job.StartTime.0.with_timezone(&chrono::Utc),
            status: job.Status,
            status_descriptions: job.StatusDescriptions,
            time_of_last_state_change: job.TimeOfLastStateChange.0.with_timezone(&chrono::Utc),
            time_submitted: job.TimeSubmitted.0.with_timezone(&chrono::Utc),
            time_before_removal: job.TimeBeforeRemoval.0,
            until_time: job.UntilTime.map(|value| value.0.with_timezone(&chrono::Utc)),
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
                .ok_or_else(|| wmi::WMIError::ConvertVariantError(format!("Related object {instance_id} not found").into()))?;
            let authority = job_path.split_once(':').map(|(authority, _)| authority).unwrap_or_default();
                let key = if relation == "Msvm_VirtualSystemReferencePoint" {
                    let virtual_system_identifier = related_object
                        .get("VirtualSystemIdentifier")
                        .and_then(serde_json::Value::as_str)
                        .ok_or_else(|| wmi::WMIError::ConvertVariantError("Related reference point has no VirtualSystemIdentifier".into()))?;
                    format!(r#"InstanceID="{instance_id}",VirtualSystemIdentifier="{virtual_system_identifier}""#)
                } else {
                    format!(r#"InstanceID="{instance_id}""#)
                };
            related_object.insert(
                "__Path".to_owned(),
                    serde_json::Value::String(format!(r#"{authority}:{relation}.{key}"#)),
            );
            related_object
        };
        serde_json::from_value(serde_json::Value::Object(related_object.into_iter().collect())).map_err(|error| {
            wmi::WMIError::ConvertVariantError(format!("Failed to deserialize related object: {error}").into())
        })
    }
}