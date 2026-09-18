use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) enum AvailableRequestedState {
    Enabled,
    Disabled,
    ShutDown,
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
            2 => Ok(Self::Enabled),
            3 => Ok(Self::Disabled),
            4 => Ok(Self::ShutDown),
            6 => Ok(Self::Offline),
            7 => Ok(Self::Test),
            8 => Ok(Self::Defer),
            9 => Ok(Self::Quiesce),
            10 => Ok(Self::Reboot),
            11 => Ok(Self::Reset),
            12..=32767 => Ok(Self::DMTFReserved(value)),
            _ => Err("Unsupported availablerequested state"),
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
