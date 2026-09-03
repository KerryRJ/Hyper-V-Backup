use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ReferencePointId(Uuid);

impl ReferencePointId {
    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

impl std::fmt::Display for ReferencePointId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<Uuid> for ReferencePointId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
