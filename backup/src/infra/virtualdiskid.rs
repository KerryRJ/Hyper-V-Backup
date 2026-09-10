use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct VirtualDiskId(Uuid);

impl VirtualDiskId {
    pub(crate) fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl AsRef<Uuid> for VirtualDiskId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for VirtualDiskId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<VirtualDiskId> for Uuid {
    fn from(value: VirtualDiskId) -> Self {
        value.0
    }
}

impl Display for VirtualDiskId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
