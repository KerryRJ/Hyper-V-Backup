use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct SequenceNumber(u16);

impl SequenceNumber {
    pub(crate) fn new(value: u16) -> Self {
        Self(value)
    }
}

impl AsRef<u16> for SequenceNumber {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for SequenceNumber {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<SequenceNumber> for u16 {
    fn from(value: SequenceNumber) -> Self {
        value.0
    }
}

impl Display for SequenceNumber {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
