use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ResilientChangeTrackingId(Uuid);

impl ResilientChangeTrackingId {
    pub(crate) fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl AsRef<Uuid> for ResilientChangeTrackingId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for ResilientChangeTrackingId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<ResilientChangeTrackingId> for Uuid {
    fn from(value: ResilientChangeTrackingId) -> Self {
        value.0
    }
}

impl Display for ResilientChangeTrackingId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
