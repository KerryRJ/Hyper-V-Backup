use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum RequestedState { 
    NotApplicable,
}

impl TryFrom<u16> for RequestedState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            12 => Ok(RequestedState::NotApplicable),
            _ => Err("Unsupported requested state"),
        }
    }
}

impl<'de> Deserialize<'de> for RequestedState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}