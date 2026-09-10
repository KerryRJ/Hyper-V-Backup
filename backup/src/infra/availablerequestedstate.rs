use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum AvailableRequestedState {
    Enabled,
    Disabled,
    ShuttingDown,
    Offline,
    Test,
    Defer,
    Quiesce,
    Reboot,
    Reset,
    DMTFReserved(u16),
}

impl TryFrom<u16> for AvailableRequestedState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(AvailableRequestedState::Enabled),
            3 => Ok(AvailableRequestedState::Disabled),
            4 => Ok(AvailableRequestedState::ShuttingDown),
            6 => Ok(AvailableRequestedState::Offline),
            7 => Ok(AvailableRequestedState::Test),
            8 => Ok(AvailableRequestedState::Defer),
            9 => Ok(AvailableRequestedState::Quiesce),
            10 => Ok(AvailableRequestedState::Reboot),
            11 => Ok(AvailableRequestedState::Reset),
            12..=32767 => Ok(AvailableRequestedState::DMTFReserved(value)),
            _ => Err("Unsupported requested state"),
        }
    }
}

impl<'de> Deserialize<'de> for AvailableRequestedState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}