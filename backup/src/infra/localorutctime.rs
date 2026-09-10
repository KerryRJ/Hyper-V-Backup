use serde::Deserialize;

#[derive(Debug)]
pub(super) enum LocalOrUtcTime {
    Local,
    Utc,
}

impl TryFrom<u16> for LocalOrUtcTime {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LocalOrUtcTime::Local),
            2 => Ok(LocalOrUtcTime::Utc),
            _ => Err("Unsupported local or UTC time"),
        }
    }
}

impl<'de> Deserialize<'de> for LocalOrUtcTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}