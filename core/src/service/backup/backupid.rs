use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct BackupId(Uuid);

impl BackupId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for BackupId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<Uuid> for BackupId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<BackupId> for Uuid {
    fn from(value: BackupId) -> Self {
        value.0
    }
}
