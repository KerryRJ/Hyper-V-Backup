use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct ParentId(Uuid);

impl ParentId {
    pub(crate) fn parse_str(value: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }

    pub(crate) fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl TryFrom<String> for ParentId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse_str(&value)
    }
}

impl AsRef<Uuid> for ParentId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for ParentId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
