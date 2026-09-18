use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum UserSnapshotType {
    Disable,                  // 2
    ProductionFallbackToTest, // 3
    ProductionNoFallback,     // 4
    Test,                     // 5
}

impl TryFrom<u16> for UserSnapshotType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Disable),
            3 => Ok(Self::ProductionFallbackToTest),
            4 => Ok(Self::ProductionNoFallback),
            5 => Ok(Self::Test),
            _ => Err("User snapshot type must be Disable (2), ProductionFallbackToTest (3), ProductionNoFallback (4) or Test (5)"),
        }
    }
}

impl<'de> Deserialize<'de> for UserSnapshotType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<&UserSnapshotType> for u16 {
    fn from(value: &UserSnapshotType) -> Self {
        match value {
            UserSnapshotType::Disable => 2,
            UserSnapshotType::ProductionFallbackToTest => 3,
            UserSnapshotType::ProductionNoFallback => 4,
            UserSnapshotType::Test => 5,
        }
    }
}
