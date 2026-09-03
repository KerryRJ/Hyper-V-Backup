use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use tokio::sync::RwLock;

use crate::actor::backup::BackupActor;
use crate::model::{BackupSchedule, Error, ScheduleId};
use crate::service::backup::{BackupId, BackupRequest, ReferencePointBackupOptions};
use crate::service::referencepointservice::ReferencePointCreateRequest;

pub struct Scheduler {
    actor: BackupActor,
    schedules: Arc<RwLock<HashMap<ScheduleId, BackupSchedule>>>,
    reference_points: Arc<RwLock<HashMap<ScheduleId, (ReferencePointCreateRequest, ReferencePointBackupOptions)>>>,
}

impl Scheduler {
    pub fn new(actor: BackupActor) -> Self {
        let schedules = Arc::new(RwLock::new(HashMap::<ScheduleId, BackupSchedule>::new()));
        let reference_points = Arc::new(RwLock::new(HashMap::<ScheduleId, (ReferencePointCreateRequest, ReferencePointBackupOptions)>::new()));
        let scheduler = schedules.clone();
        let scheduled_reference_points = reference_points.clone();
        let scheduled_actor = actor.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let now = Utc::now();
                let due = Self::collect_due(&scheduler, &scheduled_reference_points, now).await;
                for (request, reference_point) in due {
                    let _ = match reference_point {
                        Some((reference_point, options)) => scheduled_actor.start_with_reference_point(request, reference_point, options).await,
                        None => scheduled_actor.start(request).await,
                    };
                }
            }
        });

        Self { actor, schedules, reference_points }
    }

    pub async fn schedule(&self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>) -> Result<ScheduleId, Error> {
        Self::validate_request(&request)?;
        if repeat_every.is_some_and(|interval| interval <= Duration::zero()) {
            return Err(Error::InvalidBackupSchedule("repeat interval must be positive"));
        }
        let schedule = BackupSchedule::new(request.virtual_machine_id, request.destination, first_run_at, repeat_every);
        let id = schedule.id;
        self.schedules.write().await.insert(id, schedule);
        Ok(id)
    }

    pub async fn schedule_now(&self, request: BackupRequest, repeat_every: Option<Duration>) -> Result<(ScheduleId, BackupId), Error> {
        let schedule_id = self.schedule(request, Utc::now(), repeat_every).await?;
        let backup_id = self.trigger_schedule(schedule_id, Utc::now()).await?;
        Ok((schedule_id, backup_id))
    }

    pub async fn schedule_with_reference_point(
        &self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions,
    ) -> Result<ScheduleId, Error> {
        let id = self.schedule(request, first_run_at, repeat_every).await?;
        self.reference_points.write().await.insert(id, (reference_point, options));
        Ok(id)
    }

    pub async fn schedule_now_with_reference_point(&self, request: BackupRequest, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions) -> Result<(ScheduleId, BackupId), Error> {
        let schedule_id = self.schedule_with_reference_point(request, Utc::now(), repeat_every, reference_point, options).await?;
        let backup_id = self.trigger_schedule(schedule_id, Utc::now()).await?;
        Ok((schedule_id, backup_id))
    }

    pub async fn cancel(&self, id: ScheduleId) -> Result<(), Error> {
        let removed = self.schedules.write().await.remove(&id).map(|_| ()).ok_or(Error::InvalidBackupSchedule("schedule not found"));
        if removed.is_ok() {
            self.reference_points.write().await.remove(&id);
        }
        removed
    }

    pub async fn list(&self) -> Vec<BackupSchedule> {
        self.schedules.read().await.values().cloned().collect()
    }

    pub async fn set_enabled(&self, id: ScheduleId, enabled: bool) -> Result<(), Error> {
        let mut schedules = self.schedules.write().await;
        let schedule = schedules.get_mut(&id).ok_or(Error::InvalidBackupSchedule("schedule not found"))?;
        schedule.enabled = enabled;
        Ok(())
    }

    pub async fn trigger_due(&self, now: DateTime<Utc>) -> Result<Vec<BackupId>, Error> {
        let due = Self::collect_due(&self.schedules, &self.reference_points, now).await;
        let mut backup_ids = Vec::with_capacity(due.len());
        for (request, reference_point) in due {
            backup_ids.push(match reference_point {
                Some((reference_point, options)) => self.actor.start_with_reference_point(request, reference_point, options).await?,
                None => self.actor.start(request).await?,
            });
        }
        Ok(backup_ids)
    }

    async fn trigger_schedule(&self, id: ScheduleId, now: DateTime<Utc>) -> Result<BackupId, Error> {
        let (request, reference_point) = {
            let mut schedules = self.schedules.write().await;
            let schedule = schedules.get_mut(&id).ok_or(Error::InvalidBackupSchedule("schedule not found"))?;
            if !schedule.enabled || schedule.next_run_at > now {
                return Err(Error::InvalidBackupSchedule("schedule is not due"));
            }
            Self::advance(schedule, now);
            let request = BackupRequest {
                virtual_machine_id: schedule.virtual_machine_id,
                destination: schedule.destination.clone(),
            };
            let reference_point = self.reference_points.read().await.get(&id).cloned();
            (request, reference_point)
        };
        match reference_point {
            Some((reference_point, options)) => self.actor.start_with_reference_point(request, reference_point, options).await,
            None => self.actor.start(request).await,
        }
    }

    async fn collect_due(
        schedules: &Arc<RwLock<HashMap<ScheduleId, BackupSchedule>>>, reference_points: &Arc<RwLock<HashMap<ScheduleId, (ReferencePointCreateRequest, ReferencePointBackupOptions)>>>, now: DateTime<Utc>,
    ) -> Vec<(BackupRequest, Option<(ReferencePointCreateRequest, ReferencePointBackupOptions)>)> {
        let mut due = Vec::new();
        let mut schedules = schedules.write().await;
        for schedule in schedules.values_mut() {
            if !schedule.enabled || schedule.next_run_at > now {
                continue;
            }
            let request = BackupRequest {
                virtual_machine_id: schedule.virtual_machine_id,
                destination: schedule.destination.clone(),
            };
            due.push((schedule.id, request));
            Self::advance(schedule, now);
        }
        let reference_points = reference_points.read().await;
        due.into_iter().map(|(id, request)| (request, reference_points.get(&id).cloned())).collect()
    }

    fn advance(schedule: &mut BackupSchedule, now: DateTime<Utc>) {
        if let Some(repeat_every) = schedule.repeat_every {
            while schedule.next_run_at <= now {
                schedule.next_run_at += repeat_every;
            }
        } else {
            schedule.enabled = false;
        }
    }

    fn validate_request(request: &BackupRequest) -> Result<(), Error> {
        if request.virtual_machine_id.is_nil() {
            return Err(Error::InvalidBackupRequest("virtual machine id cannot be nil"));
        }
        if request.destination.as_os_str().is_empty() {
            return Err(Error::InvalidBackupRequest("destination cannot be empty"));
        }
        Ok(())
    }
}
