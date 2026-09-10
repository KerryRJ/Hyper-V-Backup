use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum HealthState {
    Ok,
    Other(u16),
}

impl TryFrom<u16> for HealthState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(HealthState::Ok),
            0..=4 | 6..=30 => Ok(HealthState::Other(value)),
            _ => Err("Unsupported health state"),
        }
    }
}

impl<'de> Deserialize<'de> for HealthState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}