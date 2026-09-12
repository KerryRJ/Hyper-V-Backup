use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplicationMode {
    None,
    Primary,
    Replica,
    TestReplica,
    ExtendedReplica,
}

impl TryFrom<u16> for ReplicationMode {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::Primary,
            2 => Self::Replica,
            3 => Self::TestReplica,
            4 => Self::ExtendedReplica,
            _ => return Err("invalid ReplicationMode value"),
        })
    }
}

impl<'de> Deserialize<'de> for ReplicationMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
