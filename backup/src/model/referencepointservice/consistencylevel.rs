use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsistencyLevel {
    Application,
    Crash,
}

impl TryFrom<u16> for ConsistencyLevel {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Application),
            2 => Ok(Self::Crash),
            _ => Err("Consistency level must be Application (1) or Crash (2)"),
        }
    }
}

impl From<ConsistencyLevel> for u8 {
    fn from(value: ConsistencyLevel) -> Self {
        match value {
            ConsistencyLevel::Application => 1,
            ConsistencyLevel::Crash => 2,
        }
    }
}

impl<'de> Deserialize<'de> for ConsistencyLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}