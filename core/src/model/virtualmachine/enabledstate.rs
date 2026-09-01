#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnabledState {
    Unknown,
    Other,
    Enabled,
    Disabled,
    ShuttingDown,
    NotApplicable,
    EnabledButOffline,
    InTest,
    Deferred,
    Quiesce,
    Starting,
    Paused,
}

impl TryFrom<u16> for EnabledState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Other,
            2 => Self::Enabled,
            3 => Self::Disabled,
            4 => Self::ShuttingDown,
            5 => Self::NotApplicable,
            6 => Self::EnabledButOffline,
            7 => Self::InTest,
            8 => Self::Deferred,
            9 => Self::Quiesce,
            10 => Self::Starting,
            32768 => Self::Paused,
            _ => return Err("invalid EnabledState value"),
        })
    }
}
