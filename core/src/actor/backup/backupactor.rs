use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc};
use tokio_util::sync::CancellationToken;

use crate::model::Error;
use crate::service::backup::{
    BackupFailure, BackupId, BackupRequest, BackupState, BackupStatus, ReferencePointRequest,
    StatusStore,
};
use crate::service::referencepointservice::ReferencePointService;

use super::command::Command;

#[derive(Clone)]
pub struct BackupActor {
    sender: mpsc::Sender<Command>,
    receiver: Arc<Mutex<Option<mpsc::Receiver<Command>>>>,
    statuses: StatusStore,
    cancellations: Arc<RwLock<HashMap<BackupId, CancellationToken>>>,
}

impl BackupActor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        Self {
            sender,
            receiver: Arc::new(Mutex::new(Some(receiver))),
            statuses: Arc::new(RwLock::new(HashMap::new())),
            cancellations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn start(&self, request: BackupRequest) -> Result<BackupId, Error> {
        let id = BackupId::new_v4();
        self.statuses.write().await.insert(
            id,
            BackupStatus {
                state: BackupState::Queued,
                progress: 0,
            },
        );
        self.cancellations
            .write()
            .await
            .insert(id, CancellationToken::new());
        self.sender
            .send(Command::Start { id, request })
            .await
            .map_err(|_| Error::InvalidBackupRequest("backup actor is stopped"))?;
        Ok(id)
    }

    pub(crate) async fn start_with_reference_point(
        &self,
        request: BackupRequest,
        reference_point: ReferencePointRequest,
    ) -> Result<BackupId, Error> {
        let id = BackupId::new_v4();
        self.statuses.write().await.insert(
            id,
            BackupStatus {
                state: BackupState::Queued,
                progress: 0,
            },
        );
        self.cancellations
            .write()
            .await
            .insert(id, CancellationToken::new());
        self.sender
            .send(Command::StartWithReferencePoint {
                id,
                request,
                reference_point,
            })
            .await
            .map_err(|_| Error::InvalidBackupRequest("backup actor is stopped"))?;
        Ok(id)
    }

    pub(crate) async fn status(&self, id: BackupId) -> Option<BackupStatus> {
        self.statuses.read().await.get(&id).cloned()
    }

    pub(crate) async fn cancel(&self, id: BackupId) -> Result<(), Error> {
        let cancellations = self.cancellations.read().await;
        cancellations
            .get(&id)
            .ok_or(Error::BackupNotFound(id))?
            .cancel();
        Ok(())
    }

    pub async fn run(self) -> Result<(), Error> {
        let Some(mut receiver) = self.receiver.lock().await.take() else {
            return Err(Error::InvalidBackupRequest(
                "backup actor is already running",
            ));
        };
        while let Some(command) = receiver.recv().await {
            match command {
                Command::Start { id, request } => self.process(id, request).await,
                Command::StartWithReferencePoint {
                    id,
                    request,
                    reference_point,
                } => self.process_with_reference_point(id, request, reference_point).await,
            }
        }
        Ok(())
    }

    async fn process(&self, id: BackupId, request: BackupRequest) {
        self.set_status(
            id,
            BackupStatus {
                state: BackupState::Running,
                progress: 0,
            },
        )
        .await;
        let cancellation = self.cancellations.read().await.get(&id).cloned();
        let Some(cancellation) = cancellation else {
            self.set_status(
                id,
                BackupStatus {
                    state: BackupState::Failed(BackupFailure::Internal),
                    progress: 0,
                },
            )
            .await;
            return;
        };

        let result = async {
            if cancellation.is_cancelled() {
                return Err(Error::BackupCancelled);
            }
            tokio::fs::create_dir_all(&request.destination).await?;
            Err(Error::BackupBackendUnavailable)
        }
        .await;

        let state = match result {
            Ok(()) => BackupState::Completed,
            Err(Error::BackupCancelled) => BackupState::Cancelled,
            Err(error) => BackupState::Failed(Self::failure_for(error)),
        };
        self.set_status(id, BackupStatus { state, progress: 0 })
            .await;
        self.cancellations.write().await.remove(&id);
    }

    async fn process_with_reference_point(
        &self,
        id: BackupId,
        request: BackupRequest,
        reference_point_request: ReferencePointRequest,
    ) {
        self.set_status(
            id,
            BackupStatus {
                state: BackupState::Running,
                progress: 0,
            },
        )
        .await;
        let cancellation = self.cancellations.read().await.get(&id).cloned();
        let Some(cancellation) = cancellation else {
            self.set_status(
                id,
                BackupStatus {
                    state: BackupState::Failed(BackupFailure::Internal),
                    progress: 0,
                },
            )
            .await;
            return;
        };

        let result = async {
            if cancellation.is_cancelled() {
                return Err(Error::BackupCancelled);
            }
            Self::run_reference_point_backup(request, reference_point_request).await
        }
        .await;

        let state = match result {
            Ok(()) => BackupState::Completed,
            Err(Error::BackupCancelled) => BackupState::Cancelled,
            Err(error) => BackupState::Failed(Self::failure_for(error)),
        };
        self.set_status(id, BackupStatus { state, progress: 0 })
            .await;
        self.cancellations.write().await.remove(&id);
    }

    async fn run_reference_point_backup(
        request: BackupRequest,
        reference_point_request: ReferencePointRequest,
    ) -> Result<(), Error> {
        tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| Error::InvalidBackupRequest("failed to create WMI runtime"))?;
            runtime.block_on(async move {
                let reference_points = ReferencePointService::new().await?;
                let reference_point = reference_points
                    .create_reference_point(
                        &reference_point_request.affected_system,
                        &reference_point_request.reference_point_settings,
                        reference_point_request.reference_point_type,
                        &reference_point_request.resulting_reference_point,
                    )
                    .await?;
                let backup_result = async {
                    tokio::fs::create_dir_all(&request.destination).await?;
                    Err(Error::BackupBackendUnavailable)
                }
                .await;
                let cleanup_result = reference_points
                    .cleanup_reference_point(
                        &reference_point,
                        reference_point_request.retain_for_incremental,
                    )
                    .await;
                match (backup_result, cleanup_result) {
                    (Err(error), _) => Err(error),
                    (Ok(()), Err(error)) => Err(error),
                    (Ok(()), Ok(())) => Ok(()),
                }
            })
        })
        .await
        .map_err(|_| Error::InvalidBackupRequest("reference-point worker stopped"))?
    }

    async fn set_status(&self, id: BackupId, status: BackupStatus) {
        self.statuses.write().await.insert(id, status);
    }

    fn failure_for(error: Error) -> BackupFailure {
        match error {
            Error::InvalidBackupRequest(_) => BackupFailure::InvalidRequest,
            Error::VirtualMachineNotFound(_) => BackupFailure::VirtualMachineNotFound,
            Error::Io(_) => BackupFailure::Io,
            Error::Wmi(_) => BackupFailure::Wmi,
            Error::BackupBackendUnavailable => BackupFailure::BackendUnavailable,
            _ => BackupFailure::Internal,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[tokio::test]
    async fn starts_without_status_for_unknown_backup() {
        let actor = BackupActor::new();

        assert_eq!(actor.status(BackupId::new_v4()).await, None);
    }

    #[tokio::test]
    async fn rejects_cancellation_for_unknown_backup() {
        let actor = BackupActor::new();

        assert!(matches!(
            actor.cancel(BackupId::new_v4()).await,
            Err(Error::BackupNotFound(_))
        ));
    }

    #[tokio::test]
    async fn cancelled_backup_does_not_start_processing() {
        let actor = BackupActor::new();
        let request = BackupRequest {
            virtual_machine_id: BackupId::new_v4(),
            destination: PathBuf::from("backup"),
        };
        let id = actor.start(request.clone()).await.unwrap();

        actor.cancel(id).await.unwrap();
        actor.process(id, request).await;

        assert_eq!(
            actor.status(id).await,
            Some(BackupStatus {
                state: BackupState::Cancelled,
                progress: 0,
            })
        );
    }

    #[test]
    fn maps_backend_error_to_typed_failure() {
        assert_eq!(
            BackupActor::failure_for(Error::BackupBackendUnavailable),
            BackupFailure::BackendUnavailable
        );
    }

    #[tokio::test]
    async fn run_can_only_be_started_once() {
        let actor = BackupActor::new();
        let worker = actor.clone();
        let worker_handle = tokio::spawn(worker.run());
        tokio::task::yield_now().await;

        let result = actor.run().await;

        assert!(matches!(
            result,
            Err(Error::InvalidBackupRequest(
                "backup actor is already running"
            ))
        ));
        worker_handle.abort();
    }
}
