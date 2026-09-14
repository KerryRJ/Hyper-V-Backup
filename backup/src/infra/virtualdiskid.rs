use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct VirtualDiskId(String);

impl VirtualDiskId {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for VirtualDiskId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Display for VirtualDiskId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
