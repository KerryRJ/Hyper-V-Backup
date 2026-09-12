use serde::Deserialize;

#[derive(Debug)]
pub(super) enum TransitioningState {
    Unknown,
    Enabled,
    Disabled,
    ShuttingDown,
    NoChange,
    Offline,
    Test,
    Defer,
    Quiesce,
    Reboot,
    Reset,
    NotApplicable,
    DMTFReserved(u16),
}

impl TryFrom<u16> for TransitioningState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            2 => Ok(Self::Enabled),
            3 => Ok(Self::Disabled),
            4 => Ok(Self::ShuttingDown),
            5 => Ok(Self::NoChange),
            6 => Ok(Self::Offline),
            7 => Ok(Self::Test),
            8 => Ok(Self::Defer),
            9 => Ok(Self::Quiesce),
            10 => Ok(Self::Reboot),
            11 => Ok(Self::Reset),
            12 => Ok(Self::NotApplicable),
            13..32767 => Ok(Self::DMTFReserved(value)),
            _ => Err("Unsupported transitioning state"),
        }
    }
}

impl<'de> Deserialize<'de> for TransitioningState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
