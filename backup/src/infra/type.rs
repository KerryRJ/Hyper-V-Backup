use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum Type {
    Fixed,
    Dynamic,
    Differencing,
}

impl TryFrom<u16> for Type {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Fixed),
            3 => Ok(Self::Dynamic),
            4 => Ok(Self::Differencing),
            _ => Err("Unsupported type"),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
