use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct VirtualMachineId(Uuid);

impl VirtualMachineId {
    pub fn new(uuid: Uuid) -> Option<Self> {
        (!uuid.is_nil()).then_some(Self(uuid))
    }

    pub fn parse_str(input: &str) -> Result<Self, VitualMachineIdError> {
        Self::new(Uuid::parse_str(input)?).ok_or(VitualMachineIdError::Nil)
    }
}

impl Display for VirtualMachineId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl AsRef<Uuid> for VirtualMachineId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for VirtualMachineId {
    fn from(value: Uuid) -> Self {
        Self::new(value).expect("VirtualMachineId cannot be created from a nil UUID")
    }
}

impl From<VirtualMachineId> for Uuid {
    fn from(value: VirtualMachineId) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for VirtualMachineId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid = Uuid::deserialize(deserializer)?;
        Self::new(uuid).ok_or_else(|| serde::de::Error::custom(VitualMachineIdError::Nil))
    }
}

#[derive(Debug)]
pub enum VitualMachineIdError {
    InvalidUuid(uuid::Error),
    Nil,
}

impl Display for VitualMachineIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUuid(error) => error.fmt(formatter),
            Self::Nil => formatter.write_str("VmId cannot be nil"),
        }
    }
}

impl std::error::Error for VitualMachineIdError {}

impl From<uuid::Error> for VitualMachineIdError {
    fn from(error: uuid::Error) -> Self {
        Self::InvalidUuid(error)
    }
}