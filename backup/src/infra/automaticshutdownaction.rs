#[derive(Debug, Clone)]
pub(crate) enum AutomaticShutdownAction {
    TurnOff,   // 2
    SaveState, // 3
    Shutdown,  // 4
    DMTF(u16),
}

impl TryFrom<u16> for AutomaticShutdownAction {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::TurnOff),
            3 => Ok(Self::SaveState),
            4 => Ok(Self::Shutdown),
            5..=32768 => Ok(Self::DMTF(value)),
            _ => Err("Automatic shutdown action must be TurnOff (2), SaveState (3), Shutdown (4), or a DMTF value from 5 through 32768"),
        }
    }
}

impl From<&AutomaticShutdownAction> for u16 {
    fn from(value: &AutomaticShutdownAction) -> Self {
        match value {
            AutomaticShutdownAction::TurnOff => 2,
            AutomaticShutdownAction::SaveState => 3,
            AutomaticShutdownAction::Shutdown => 4,
            AutomaticShutdownAction::DMTF(value) => *value,
        }
    }
}
