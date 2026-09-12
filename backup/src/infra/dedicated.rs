use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum Dedicated { 
    NotDedicated,
}

impl TryFrom<u16> for Dedicated {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotDedicated),
            _ => Err("Unsupported dedicated state"),
        }
    }
}

impl<'de> Deserialize<'de> for Dedicated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}