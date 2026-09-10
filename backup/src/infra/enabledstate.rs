use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum EnabledState { 
    Enabled,
}

impl TryFrom<u16> for EnabledState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(EnabledState::Enabled),
            _ => Err("Unsupported enabled state"),
        }
    }
}

impl<'de> Deserialize<'de> for EnabledState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}