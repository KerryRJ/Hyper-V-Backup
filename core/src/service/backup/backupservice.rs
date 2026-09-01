use std::sync::Arc;

use crate::actor::backup::BackupActor;
use crate::model::{Error, Host};

use super::{BackupId, BackupRequest, BackupStatus};

pub struct BackupService {
    host: Arc<Host>,
    actor: BackupActor,
}

impl BackupService {
    pub fn new(host: Arc<Host>) -> Self {
        let actor = BackupActor::new();
        tokio::spawn(actor.clone().run());
        Self { host, actor }
    }

    pub async fn start(&self, request: BackupRequest) -> Result<BackupId, Error> {
        if request.virtual_machine_id.is_nil() {
            return Err(Error::InvalidBackupRequest(
                "virtual machine id cannot be nil",
            ));
        }
        if request.destination.as_os_str().is_empty() {
            return Err(Error::InvalidBackupRequest("destination cannot be empty"));
        }
        self.host
            .get_virtual_machine(request.virtual_machine_id)
            .await?;
        self.actor.start(request).await
    }

    pub async fn status(&self, id: BackupId) -> Option<BackupStatus> {
        self.actor.status(id).await
    }

    pub async fn cancel(&self, id: BackupId) -> Result<(), Error> {
        self.actor.cancel(id).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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

        assert!(matches!(
            result,
            Err(Error::InvalidBackupRequest(
                "virtual machine id cannot be nil"
            ))
        ));
    }

    #[tokio::test]
    async fn rejects_empty_destination() {
        let result = service()
            .start(BackupRequest {
                virtual_machine_id: BackupId::new_v4(),
                destination: PathBuf::new(),
            })
            .await;

        assert!(matches!(
            result,
            Err(Error::InvalidBackupRequest("destination cannot be empty"))
        ));
    }
}
