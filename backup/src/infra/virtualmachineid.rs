use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct VirtualMachineId(Uuid);

impl VirtualMachineId {
    pub(crate) fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl AsRef<Uuid> for VirtualMachineId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for VirtualMachineId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<VirtualMachineId> for Uuid {
    fn from(value: VirtualMachineId) -> Self {
        value.0
    }
}

impl Display for VirtualMachineId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
