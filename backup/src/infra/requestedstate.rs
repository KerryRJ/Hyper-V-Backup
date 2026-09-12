use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum RequestedState {
    Enabled,
    Disabled,
    ShutDown,
    NoChange,
    Offline,
    Test,
    Reboot,
    Reset, 
    NotApplicable,
}

impl TryFrom<u16> for RequestedState {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Enabled),
            3 => Ok(Self::Disabled),
            4 => Ok(Self::ShutDown),
            5 => Ok(Self::NoChange),
            6 => Ok(Self::Offline),
            7 => Ok(Self::Test),
            10 => Ok(Self::Reboot),
            11 => Ok(Self::Reset),
            12 => Ok(Self::NotApplicable),
            _ => Err(format!("Unsupported requested state {}", value)),
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