use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitioningToState {
    Unknown,
    Enabled,
    Disabled,
    ShutDown,
    NoChange,
    Offline,
    InTest,
    Deferred,
    Quiesce,
    Reboot,
    Reset,
    NotApplicable,
    Saving,
    Pausing,
    Resuming,
    FastSaving,
}

impl TryFrom<u16> for TransitioningToState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            2 => Self::Enabled,
            3 => Self::Disabled,
            4 => Self::ShutDown,
            5 => Self::NoChange,
            6 => Self::Offline,
            7 => Self::InTest,
            8 => Self::Deferred,
            9 => Self::Quiesce,
            10 => Self::Reboot,
            11 => Self::Reset,
            12 => Self::NotApplicable,
            32773 => Self::Saving,
            32776 => Self::Pausing,
            32777 => Self::Resuming,
            32780 => Self::FastSaving,
            _ => return Err("invalid TransitioningToState value"),
        })
    }
}

impl<'de> Deserialize<'de> for TransitioningToState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
