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
            0 => Ok(TransitioningState::Unknown),
            2 => Ok(TransitioningState::Enabled),
            3 => Ok(TransitioningState::Disabled),
            4 => Ok(TransitioningState::ShuttingDown),
            5 => Ok(TransitioningState::NoChange),
            6 => Ok(TransitioningState::Offline),
            7 => Ok(TransitioningState::Test),
            8 => Ok(TransitioningState::Defer),
            9 => Ok(TransitioningState::Quiesce),
            10 => Ok(TransitioningState::Reboot),
            11 => Ok(TransitioningState::Reset),
            12 => Ok(TransitioningState::NotApplicable),
            13..32767 => Ok(TransitioningState::DMTFReserved(value)),
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
