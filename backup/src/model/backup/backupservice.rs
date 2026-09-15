use crate::infra::*;
use crate::model::*;
use chrono::{Local, Utc};
use super::*;

// use chrono::{DateTime, Duration, Utc};
// use crate::actor::backup::BackupActor;
// use crate::model::{BackupSchedule, Error, Host, ScheduleId};
// use crate::service::referencepointservice::ReferencePointCreateRequest;
// use crate::service::scheduler::Scheduler;

// use super::{BackupId, BackupRequest, BackupStatus, ReferencePointBackupOptions};

pub struct BackupService {
    snapshot_service: VirtualSystemSnapshotService,
    management_service: VirtualSystemManagementService,
    // actor: BackupActor,
    // scheduler: Scheduler,
}

impl BackupService {
    pub async fn backup(
        &self,
        request: BackupRequest,
    ) -> Result<BackupResult, Error> {
        let virtual_machine = request.virtual_machine;
        let virtual_machine_name = virtual_machine.element_name.clone();
        let is_incremental = request.differential_backup_base.is_some();
        log::info!(
            "Starting {} backup for virtual machine name {}",
            if is_incremental { "incremental" } else { "full" },
            virtual_machine_name
        );
        let snapshot_settings = SnapshotSettings {
            properties: vec![
                SnapshotProperty {
                    name: "ConsistencyLevel".into(),
                    value: SnapshotPropertyValue::Uint8(u8::from(request.crash_consistency)),
                },
                SnapshotProperty {
                    name: "IgnoreNonSnapshottableDisks".into(),
                    value: SnapshotPropertyValue::Boolean(true),
                },
            ],
        };
        let snapshot = self
            .snapshot_service
            .create(
                &virtual_machine,
                Some((&snapshot_settings).into()),
                request.snapshot_type,
                None,
            )
            .await?;

        log::debug!(
            "Created backup snapshot {} for virtual machine name {}",
            snapshot.path.as_str(),
            virtual_machine_name
        );

        let mut export_settings = ExportSettings {
            properties: vec![
                ExportProperty {
                    name: "CaptureLiveState".into(),
                    value: ExportPropertyValue::Uint8(0),
                },
                ExportProperty {
                    name: "CopySnapshotConfiguration".into(),
                    value: ExportPropertyValue::Uint8(CopySnapshotConfiguration::ExportOneSnapshotForBackup as u8),
                },
                ExportProperty {
                    name: "CopyVmRuntimeInformation".into(),
                    value: ExportPropertyValue::Boolean(false),
                },
                ExportProperty {
                    name: "CopyVmStorage".into(),
                    value: ExportPropertyValue::Boolean(true),
                },
                ExportProperty {
                    name: "CreateVmExportSubdirectory".into(),
                    value: ExportPropertyValue::Boolean(false),
                },
            ]
        };
        export_settings.properties.push(ExportProperty {
            name: "SnapshotVirtualSystem".into(),
            value: ExportPropertyValue::String(snapshot.path.as_str().to_owned()),
        });
        let export_prefix = if request.differential_backup_base.is_some() {
            "i"
        } else {
            "f"
        };
        if let Some(reference_point) = request.differential_backup_base {
            log::debug!(
                "Using differential backup base {}",
                reference_point.path.as_str()
            );

            let current_reference_points = virtual_machine.get_reference_points().await;
            log::debug!(
                "Current reference points on virtual machine {}: {current_reference_points:#?}",
                virtual_machine_name
            );

            export_settings.properties.push(ExportProperty {
                name: "DifferentialBackupBase".into(),
                value: ExportPropertyValue::String(reference_point.path.as_str().to_owned()),
            });
        }
        log::debug!("Export settings: {export_settings:#?}");
        let export_directory = request.destination.join(format!(
            "{export_prefix}-{}",
            Local::now().format("%Y%m%d-%H%M%S")
        ));
        log::info!(
            "Exporting backup for virtual machine name {} to {}",
            virtual_machine_name,
            export_directory.display(),
        );
        let result = self
            .management_service
            .export_system_definition(
                virtual_machine.clone(),
                export_directory,
                Some(VirtualSystemExportSettingDataIn::from(&export_settings)),
            )
            .await;
        match result {
            Ok(_) => {
                log::debug!(
                    "Export completed for virtual machine name {}; converting snapshot {} to a reference point",
                    virtual_machine_name,
                    snapshot.path.as_str()
                );
                let reference_point_settings = ReferencePointSettings {
                    properties: vec![
                        ReferencePointProperty {
                            name: "ConsistencyLevel".into(),
                            value: ReferencePointPropertyValue::Uint8(u8::from(request.crash_consistency)),
                        },
                    ],
                };
                let reference_point = self.snapshot_service
                    .convert_to_reference_point(
                        snapshot,
                        Some((&reference_point_settings).into()),
                        None,
                    )
                    .await?;
                log::info!(
                    "Completed {} backup for virtual machine name {} with reference point {}",
                    if is_incremental { "incremental" } else { "full" },
                    virtual_machine_name,
                    reference_point.path.as_str()
                );
                Ok(BackupResult {
                    backup_id: BackupId::new_v4(),
                    virtual_machine: virtual_machine,
                    destination: request.destination,
                    reference_point: reference_point,
                    completed_at: Utc::now(),
                })
            }
            Err(error) => {
                log::error!(
                    "Failed to export {} backup for virtual machine name {}: {}",
                    if is_incremental { "incremental" } else { "full" },
                    virtual_machine_name,
                    error
                );
                log::debug!("Destroying failed backup snapshot {}", snapshot.path.as_str());
                self.snapshot_service.destroy(snapshot).await?;
                Err(error.into())
            }
        }
    }
}

#[cfg(all(test, windows))]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[tokio::test]
    #[ignore = "requires a configured Hyper-V VM and writable export directory"]
    async fn full_then_incremental_backup_exports_vm_and_returns_results() {
        let _ = env_logger::try_init();
        let virtual_machine_name =
            std::env::var("HYPER_V_VM").expect("HYPER_V_VM must be set");
        let destination = PathBuf::from(
            std::env::var("HYPER_V_BACKUP_DESTINATION")
                .expect("HYPER_V_BACKUP_DESTINATION must be set"),
        );
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)
            .expect("Hyper-V WMI connection should be available");
        let virtual_machine = Host::new()
            .expect("Hyper-V host should be available")
            .get_virtual_machine_by_name(&virtual_machine_name)
            .await
            .expect("Hyper-V VM should be available");
        let virtual_machine_id = virtual_machine.name;
        let reference_point_service = VirtualSystemReferencePointService::new(connection.clone())
            .expect("Hyper-V reference-point service should be available");
        let service = BackupService {
            snapshot_service: VirtualSystemSnapshotService::new(connection.clone())
                .expect("Hyper-V snapshot service should be available"),
            management_service: VirtualSystemManagementService::new(connection)
                .expect("Hyper-V management service should be available"),
        };
        let backup_request = BackupRequest {
            virtual_machine: virtual_machine.clone(),
            snapshot_type: SnapshotType::VendorSpecific(32768),
            crash_consistency: ConsistencyLevel::Crash,
            destination: destination.clone(),
            differential_backup_base: None,
        };
        let full_result = service
            .backup(backup_request)
            .await
            .expect("full backup should complete");

        assert_eq!(full_result.virtual_machine.name, virtual_machine_id);
        assert_eq!(full_result.destination, destination);
        assert!(!full_result.backup_id.to_string().is_empty());
        assert!(!full_result.reference_point.path.to_string().is_empty());
        let full_reference_point = full_result.reference_point;

        let incremental_destination = destination;
        let incremental_result = service
            .backup(BackupRequest {
                virtual_machine,
                snapshot_type: SnapshotType::VendorSpecific(32768),
                crash_consistency: ConsistencyLevel::Crash,
                destination: incremental_destination.clone(),
                differential_backup_base: Some(full_reference_point.clone()),
            })
            .await
            .expect("incremental backup should complete");

        assert_eq!(incremental_result.virtual_machine.name, virtual_machine_id);
        assert_eq!(incremental_result.destination, incremental_destination);
        assert!(!incremental_result.backup_id.to_string().is_empty());
        assert!(!incremental_result.reference_point.path.to_string().is_empty());

        reference_point_service
            .destroy(full_reference_point)
            .await
            .expect("full backup reference point should be destroyed");
        reference_point_service
            .destroy(incremental_result.reference_point)
            .await
            .expect("incremental backup reference point should be destroyed");
    }
}


// impl BackupService {
//     pub fn new(host: Arc<Host>) -> Self {
//         let actor = BackupActor::new();
//         tokio::spawn(actor.clone().run());
//         let scheduler = Scheduler::new(actor.clone());
//         Self { host, actor, scheduler }
//     }

//     pub async fn start(&self, request: BackupRequest) -> Result<BackupId, Error> {
//         if request.destination.as_os_str().is_empty() {
//             return Err(Error::InvalidBackupRequest("destination cannot be empty"));
//         }
//         self.host.get_virtual_machine(request.virtual_machine_id).await?;
//         self.actor.start(request).await
//     }

//     pub async fn status(&self, id: BackupId) -> Option<BackupStatus> {
//         self.actor.status(id).await
//     }

//     pub async fn start_with_reference_point(&self, request: BackupRequest, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions) -> Result<BackupId, Error> {
//         if request.destination.as_os_str().is_empty() {
//             return Err(Error::InvalidBackupRequest("destination cannot be empty"));
//         }
//         self.host.get_virtual_machine(request.virtual_machine_id).await?;
//         self.actor.start_with_reference_point(request, reference_point, options).await
//     }

//     pub async fn cancel(&self, id: BackupId) -> Result<(), Error> {
//         self.actor.cancel(id).await
//     }

//     pub async fn schedule(&self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>) -> Result<ScheduleId, Error> {
//         self.scheduler.schedule(request, first_run_at, repeat_every).await
//     }

//     pub async fn schedule_now(&self, request: BackupRequest, repeat_every: Option<Duration>) -> Result<(ScheduleId, BackupId), Error> {
//         self.scheduler.schedule_now(request, repeat_every).await
//     }

//     pub async fn schedule_with_reference_point(
//         &self, request: BackupRequest, first_run_at: DateTime<Utc>, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions,
//     ) -> Result<ScheduleId, Error> {
//         self.scheduler.schedule_with_reference_point(request, first_run_at, repeat_every, reference_point, options).await
//     }

//     pub async fn schedule_now_with_reference_point(&self, request: BackupRequest, repeat_every: Option<Duration>, reference_point: ReferencePointCreateRequest, options: ReferencePointBackupOptions) -> Result<(ScheduleId, BackupId), Error> {
//         self.scheduler.schedule_now_with_reference_point(request, repeat_every, reference_point, options).await
//     }

//     pub async fn cancel_schedule(&self, id: ScheduleId) -> Result<(), Error> {
//         self.scheduler.cancel(id).await
//     }

//     pub async fn schedules(&self) -> Vec<BackupSchedule> {
//         self.scheduler.list().await
//     }

//     pub async fn set_schedule_enabled(&self, id: ScheduleId, enabled: bool) -> Result<(), Error> {
//         self.scheduler.set_enabled(id, enabled).await
//     }

//     pub async fn trigger_due(&self, now: DateTime<Utc>) -> Result<Vec<BackupId>, Error> {
//         self.scheduler.trigger_due(now).await
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use chrono::Duration;

//     use crate::infra::VirtualMachineId;

//     use super::*;

//     fn vm_id() -> VirtualMachineId {
//         VirtualMachineId::parse_str("11111111-1111-1111-1111-111111111111").unwrap()
//     }

//     fn service() -> BackupService {
//         BackupService::new(Arc::new(Host::new()))
//     }

//     #[tokio::test]
//     async fn rejects_empty_destination() {
//         let result = service()
//             .start(BackupRequest {
//                 virtual_machine_id: vm_id(),
//                 destination: PathBuf::new(),
//             })
//             .await;

//         assert!(matches!(result, Err(Error::InvalidBackupRequest("destination cannot be empty"))));
//     }

//     #[tokio::test]
//     async fn future_schedule_does_not_trigger_early() {
//         let service = service();
//         let request = BackupRequest {
//             virtual_machine_id: vm_id(),
//             destination: PathBuf::from("backup"),
//         };
//         let now = Utc::now();

//         service.schedule(request, now + Duration::minutes(1), None).await.unwrap();

//         assert!(service.trigger_due(now).await.unwrap().is_empty());
//     }

//     #[tokio::test]
//     async fn stores_schedule_configuration() {
//         let service = service();
//         let virtual_machine_id = vm_id();
//         let destination = PathBuf::from("backup");
//         let first_run_at = Utc::now() + Duration::minutes(5);
//         let repeat_every = Some(Duration::hours(1));

//         let id = service
//             .schedule(
//                 BackupRequest {
//                     virtual_machine_id,
//                     destination: destination.clone(),
//                 },
//                 first_run_at,
//                 repeat_every,
//             )
//             .await
//             .unwrap();

//         let schedule = service.schedules().await.into_iter().find(|schedule| schedule.id == id).unwrap();

//         assert_eq!(schedule.virtual_machine_id, virtual_machine_id);
//         assert_eq!(schedule.destination, destination);
//         assert_eq!(schedule.next_run_at, first_run_at);
//         assert_eq!(schedule.repeat_every, repeat_every);
//         assert!(schedule.enabled);
//     }

//     #[tokio::test]
//     async fn rejects_non_positive_repeat_interval() {
//         let service = service();
//         let request = BackupRequest {
//             virtual_machine_id: vm_id(),
//             destination: PathBuf::from("backup"),
//         };

//         for repeat_every in [Duration::zero(), -Duration::minutes(1)] {
//             assert!(matches!(
//                 service.schedule(request.clone(), Utc::now(), Some(repeat_every)).await,
//                 Err(Error::InvalidBackupSchedule("repeat interval must be positive"))
//             ));
//         }
//     }

//     #[tokio::test]
//     async fn one_shot_schedule_is_disabled_after_triggering() {
//         let service = service();
//         let now = Utc::now();
//         let id = service
//             .schedule(
//                 BackupRequest {
//                     virtual_machine_id: vm_id(),
//                     destination: PathBuf::from("backup"),
//                 },
//                 now,
//                 None,
//             )
//             .await
//             .unwrap();

//         assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
//         assert!(!service.schedules().await.into_iter().find(|schedule| schedule.id == id).unwrap().enabled);
//         assert!(service.trigger_due(now).await.unwrap().is_empty());
//     }

//     #[tokio::test]
//     async fn disabled_schedule_does_not_trigger_until_reenabled() {
//         let service = service();
//         let now = Utc::now();
//         let id = service
//             .schedule(
//                 BackupRequest {
//                     virtual_machine_id: vm_id(),
//                     destination: PathBuf::from("backup"),
//                 },
//                 now,
//                 Some(Duration::minutes(1)),
//             )
//             .await
//             .unwrap();

//         service.set_schedule_enabled(id, false).await.unwrap();
//         assert!(service.trigger_due(now).await.unwrap().is_empty());

//         service.set_schedule_enabled(id, true).await.unwrap();
//         assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
//     }

//     #[tokio::test]
//     async fn cancel_schedule_removes_schedule() {
//         let service = service();
//         let id = service
//             .schedule(
//                 BackupRequest {
//                     virtual_machine_id: vm_id(),
//                     destination: PathBuf::from("backup"),
//                 },
//                 Utc::now(),
//                 None,
//             )
//             .await
//             .unwrap();

//         service.cancel_schedule(id).await.unwrap();

//         assert!(service.schedules().await.into_iter().all(|schedule| schedule.id != id));
//         assert!(matches!(service.cancel_schedule(id).await, Err(Error::InvalidBackupSchedule("schedule not found"))));
//     }

//     #[tokio::test]
//     async fn triggers_multiple_due_schedules() {
//         let service = service();
//         let now = Utc::now();

//         for _ in 0..2 {
//             service
//                 .schedule(
//                     BackupRequest {
//                         virtual_machine_id: vm_id(),
//                         destination: PathBuf::from("backup"),
//                     },
//                     now,
//                     None,
//                 )
//                 .await
//                 .unwrap();
//         }

//         assert_eq!(service.trigger_due(now).await.unwrap().len(), 2);
//     }

//     #[tokio::test]
//     async fn repeating_schedule_advances_after_missed_ticks() {
//         let service = service();
//         let now = Utc::now();

//         service
//             .schedule(
//                 BackupRequest {
//                     virtual_machine_id: vm_id(),
//                     destination: PathBuf::from("backup"),
//                 },
//                 now - Duration::minutes(5),
//                 Some(Duration::minutes(1)),
//             )
//             .await
//             .unwrap();

//         assert_eq!(service.trigger_due(now).await.unwrap().len(), 1);
//         assert!(service.trigger_due(now).await.unwrap().is_empty());
//     }
// }
