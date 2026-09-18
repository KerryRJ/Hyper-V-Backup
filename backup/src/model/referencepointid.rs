use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct ReferencePointId(Uuid);

#[derive(Debug)]
pub enum ReferencePointIdError {
    InvalidUuid(uuid::Error),
    Nil,
}

impl Display for ReferencePointIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUuid(error) => error.fmt(formatter),
            Self::Nil => formatter.write_str("reference point identifier cannot be nil"),
        }
    }
}

impl std::error::Error for ReferencePointIdError {}

impl From<uuid::Error> for ReferencePointIdError {
    fn from(error: uuid::Error) -> Self {
        Self::InvalidUuid(error)
    }
}

impl ReferencePointId {
    pub fn parse_str(input: &str) -> Result<Self, ReferencePointIdError> {
        Self::try_from(Uuid::parse_str(input)?).map_err(|_| ReferencePointIdError::Nil)
    }
}

impl std::fmt::Display for ReferencePointId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl TryFrom<Uuid> for ReferencePointId {
    type Error = &'static str;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        (!value.is_nil()).then_some(Self(value)).ok_or("reference point identifier cannot be nil")
    }
}

impl<'de> serde::Deserialize<'de> for ReferencePointId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).and_then(|value| Self::try_from(value).map_err(serde::de::Error::custom))
    }
}
