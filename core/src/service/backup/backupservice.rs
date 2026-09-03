use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};

use crate::actor::backup::BackupActor;
use crate::model::{BackupSchedule, Error, Host, ScheduleId};
use crate::service::scheduler::Scheduler;

use crate::service::referencepointservice::ReferencePointCreateRequest;

use super::{BackupId, BackupRequest, BackupStatus, ReferencePointBackupOptions};

pub struct BackupService {
    host: Arc<Host>,
    actor: BackupActor,
    scheduler: Scheduler,
}

impl BackupService {
    pub fn new(host: Arc<Host>) -> Self {
        let actor = BackupActor::new();
        tokio::spawn(actor.clone().run());
        let scheduler = Scheduler::new(actor.clone());
        Self { host, actor, scheduler }
    }

    pub async fn start(&self, request: BackupRequest) -> Result<BackupId, Error> {
        if request.virtual_machine_id.is_nil() {
            return Err(Error::InvalidBackupRequest("virtual machine id cannot be nil"));
        }
        if request.destination.as_os_str().is_empty() {
            return Err(Error::InvalidBackupRequest("destination cannot be empty"));
        }
        self.host.get_virtual_machine(request.virtual_machine_id).await?;
        self.actor.start(request).await
    }

    pub async fn status(&self, id: BackupId) -> Option<BackupStatus> {
        self.actor.status(id).await
    }

    pub async fn start_with_reference_point(&self, request: BackupRequest, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions) -> Result<BackupId, Error> {
        if request.virtual_machine_id.is_nil() {
            return Err(Error::InvalidBackupRequest("virtual machine id cannot be nil"));
        }
        if request.destination.as_os_str().is_empty() {
            return Err(Error::InvalidBackupRequest("destination cannot be empty"));
        }
        self.host.get_virtual_machine(request.virtual_machine_id).await?;
        self.actor.start_with_reference_point(request, reference_point, options).await
    }

    pub async fn cancel(&self, id: BackupId) -> Result<(), Error> {
        self.actor.cancel(id).await
    }

    pub async fn schedule(&self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>) -> Result<ScheduleId, Error> {
        self.scheduler.schedule(request, first_run_at, repeat_every).await
    }

    pub async fn schedule_now(&self, request: BackupRequest, repeat_every: Option<Duration>) -> Result<(ScheduleId, BackupId), Error> {
        self.scheduler.schedule_now(request, repeat_every).await
    }

    pub async fn schedule_with_reference_point(
        &self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions,
    ) -> Result<ScheduleId, Error> {
        self.scheduler.schedule_with_reference_point(request, first_run_at, repeat_every, reference_point, options).await
    }

    pub async fn schedule_now_with_reference_point(&self, request: BackupRequest, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions) -> Result<(ScheduleId, BackupId), Error> {
        self.scheduler.schedule_now_with_reference_point(request, repeat_every, reference_point, options).await
    }

    pub async fn cancel_schedule(&self, id: ScheduleId) -> Result<(), Error> {
        self.scheduler.cancel(id).await
    }

    pub async fn schedules(&self) -> Vec<BackupSchedule> {
        self.scheduler.list().await
    }

    pub async fn set_schedule_enabled(&self, id: ScheduleId, enabled: bool) -> Result<(), Error> {
        self.scheduler.set_enabled(id, enabled).await
    }

    pub async fn trigger_due(&self, now: DateTime<Utc>) -> Result<Vec<BackupId>, Error> {
        self.scheduler.trigger_due(now).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::Duration;

    use super::*;

    fn service() -> BackupService {
        BackupService::new(Arc::new(Host::new()))
    }

    #[tokio::test]
    async fn rejects_nil_virtual_machine_id() {
        let result = service()
            .start(BackupRequest {
                virtual_machine_id: BackupId::nil(),
                destination: PathBuf::from("backup"),
            })
            .await;

        assert!(matches!(result, Err(Error::InvalidBackupRequest("virtual machine id cannot be nil"))));
    }

    #[tokio::test]
    async fn rejects_empty_destination() {
        let result = service()
            .start(BackupRequest {
                virtual_machine_id: BackupId::new_v4(),
                destination: PathBuf::new(),
            })
            .await;

        assert!(matches!(result, Err(Error::InvalidBackupRequest("destination cannot be empty"))));
    }

    #[tokio::test]
    async fn future_schedule_does_not_trigger_early() {
        let service = service();
        let request = BackupRequest {
            virtual_machine_id: BackupId::new_v4(),
            destination: PathBuf::from("backup"),
        };
        let now = Utc::now();

        service.schedule(request, now + Duration::minutes(1), None).await.unwrap();

        assert!(service.trigger_due(now).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn stores_schedule_configuration() {
        let service = service();
        let virtual_machine_id = BackupId::new_v4();
        let destination = PathBuf::from("backup");
        let first_run_at = Utc::now() + Duration::minutes(5);
        let repeat_every = Some(Duration::hours(1));

        let id = service
            .schedule(
                BackupRequest {
                    virtual_machine_id,
                    destination: destination.clone(),
                },
                first_run_at,
                repeat_every,
            )
            .await
            .unwrap();

        let schedule = service.schedules().await.into_iter().find(|schedule| schedule.id == id).unwrap();

        assert_eq!(schedule.virtual_machine_id, virtual_machine_id);
        assert_eq!(schedule.destination, destination);
        assert_eq!(schedule.next_run_at, first_run_at);
        assert_eq!(schedule.repeat_every, repeat_every);
        assert!(schedule.enabled);
    }

    #[tokio::test]
    async fn rejects_non_positive_repeat_interval() {
        let service = service();
        let request = BackupRequest {
            virtual_machine_id: BackupId::new_v4(),
            destination: PathBuf::from("backup"),
        };

        for repeat_every in [Duration::zero(), -Duration::minutes(1)] {
            assert!(matches!(
                service.schedule(request.clone(), Utc::now(), Some(repeat_every)).await,
                Err(Error::InvalidBackupSchedule("repeat interval must be positive"))
            ));
        }
    }

    #[tokio::test]
    async fn one_shot_schedule_is_disabled_after_triggering() {
        let service = service();
        let now = Utc::now();
        let id = service
            .schedule(
                BackupRequest {
                    virtual_machine_id: BackupId::new_v4(),
                    destination: PathBuf::from("backup"),
                },
                now,
                None,
            )
            .await
            .unwrap();

        assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
        assert!(!service.schedules().await.into_iter().find(|schedule| schedule.id == id).unwrap().enabled);
        assert!(service.trigger_due(now).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn disabled_schedule_does_not_trigger_until_reenabled() {
        let service = service();
        let now = Utc::now();
        let id = service
            .schedule(
                BackupRequest {
                    virtual_machine_id: BackupId::new_v4(),
                    destination: PathBuf::from("backup"),
                },
                now,
                Some(Duration::minutes(1)),
            )
            .await
            .unwrap();

        service.set_schedule_enabled(id, false).await.unwrap();
        assert!(service.trigger_due(now).await.unwrap().is_empty());

        service.set_schedule_enabled(id, true).await.unwrap();
        assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn cancel_schedule_removes_schedule() {
        let service = service();
        let id = service
            .schedule(
                BackupRequest {
                    virtual_machine_id: BackupId::new_v4(),
                    destination: PathBuf::from("backup"),
                },
                Utc::now(),
                None,
            )
            .await
            .unwrap();

        service.cancel_schedule(id).await.unwrap();

        assert!(service.schedules().await.into_iter().all(|schedule| schedule.id != id));
        assert!(matches!(service.cancel_schedule(id).await, Err(Error::InvalidBackupSchedule("schedule not found"))));
    }

    #[tokio::test]
    async fn triggers_multiple_due_schedules() {
        let service = service();
        let now = Utc::now();

        for _ in 0..2 {
            service
                .schedule(
                    BackupRequest {
                        virtual_machine_id: BackupId::new_v4(),
                        destination: PathBuf::from("backup"),
                    },
                    now,
                    None,
                )
                .await
                .unwrap();
        }

        assert_eq!(service.trigger_due(now).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn repeating_schedule_advances_after_missed_ticks() {
        let service = service();
        let now = Utc::now();

        service
            .schedule(
                BackupRequest {
                    virtual_machine_id: BackupId::new_v4(),
                    destination: PathBuf::from("backup"),
                },
                now - Duration::minutes(5),
                Some(Duration::minutes(1)),
            )
            .await
            .unwrap();

        assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
        assert!(service.trigger_due(now).await.unwrap().is_empty());
    }
}
