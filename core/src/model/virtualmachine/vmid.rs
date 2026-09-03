use std::fmt::{Display, Formatter};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct VmId(Uuid);

#[derive(Debug)]
pub enum VmIdError {
    InvalidUuid(uuid::Error),
    Nil,
}

impl Display for VmIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUuid(error) => error.fmt(formatter),
            Self::Nil => formatter.write_str("VmId cannot be nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nil_values() {
        assert!(VmId::new(Uuid::nil()).is_none());
        assert!(matches!(VmId::parse_str("00000000-0000-0000-0000-000000000000"), Err(VmIdError::Nil)));
    }
}

impl std::error::Error for VmIdError {}

impl From<uuid::Error> for VmIdError {
    fn from(error: uuid::Error) -> Self {
        Self::InvalidUuid(error)
    }
}

impl VmId {
    pub fn new(uuid: Uuid) -> Option<Self> {
        (!uuid.is_nil()).then_some(Self(uuid))
    }

    pub fn parse_str(input: &str) -> Result<Self, VmIdError> {
        Self::new(Uuid::parse_str(input)?).ok_or(VmIdError::Nil)
    }
}

impl<'de> Deserialize<'de> for VmId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid = Uuid::deserialize(deserializer)?;
        Self::new(uuid).ok_or_else(|| serde::de::Error::custom(VmIdError::Nil))
    }
}

impl AsRef<Uuid> for VmId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<VmId> for Uuid {
    fn from(value: VmId) -> Self {
        value.0
    }
}

impl std::fmt::Display for VmId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
