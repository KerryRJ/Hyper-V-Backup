#[derive(Debug, Clone)]
pub(crate) enum AutomaticStartupAction {
    None,   // 2
    RestartIfPreviouslyActive,  // 3
    AlwaysStartup,  // 4
    DMTF(u16),
}

impl TryFrom<u16> for AutomaticStartupAction {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::None),
            3 => Ok(Self::RestartIfPreviouslyActive),
            4 => Ok(Self::AlwaysStartup),
            5..=32768 => Ok(Self::DMTF(value)),
            _ => Err("Automatic startup action must be None (2), RestartIfPreviouslyActive (3), AlwaysStartup (4), or a DMTF value from 5 through 32768"),
        }
    }
}

impl From<&AutomaticStartupAction> for u16 {
    fn from(value: &AutomaticStartupAction) -> Self {
        match value {
            AutomaticStartupAction::None => 2,
            AutomaticStartupAction::RestartIfPreviouslyActive => 3,
            AutomaticStartupAction::AlwaysStartup => 4,
            AutomaticStartupAction::DMTF(value) => *value,
        }
    }
}