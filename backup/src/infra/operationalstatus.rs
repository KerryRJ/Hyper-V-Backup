use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum OperationalStatus { 
    Ok,
}

impl TryFrom<u16> for OperationalStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(OperationalStatus::Ok),
            _ => Err("Unsupported operational status"),
        }
    }
}

impl<'de> Deserialize<'de> for OperationalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}