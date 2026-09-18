use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct SnapshotId(Uuid);

impl SnapshotId {
    pub(crate) fn parse_str(value: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }

    pub(crate) fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl TryFrom<String> for SnapshotId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse_str(&value)
    }
}

impl AsRef<Uuid> for SnapshotId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for SnapshotId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
