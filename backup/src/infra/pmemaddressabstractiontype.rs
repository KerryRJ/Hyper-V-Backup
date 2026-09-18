use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum PmemAddressAbstractionType {
    None,
    Btt,
    Unknown,
}

impl TryFrom<u16> for PmemAddressAbstractionType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Btt),
            65535 => Ok(Self::Unknown),
            _ => Err("Unsupported pmem address abstraction type"),
        }
    }
}

impl<'de> Deserialize<'de> for PmemAddressAbstractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
