#[derive(Debug, Clone)]
pub(crate) enum AutomaticRecoveryAction {
    None, // 2
    Restart, // 3
    RevertToSnapshot, // 4
    DMTF(u16),
}

impl TryFrom<u16> for AutomaticRecoveryAction {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::None),
            3 => Ok(Self::Restart),
            4 => Ok(Self::RevertToSnapshot),
            5..=32768 => Ok(Self::DMTF(value)),
            _ => Err("Automatic recovery action must be None (2), Restart (3), RevertToSnapshot (4), or a DMTF value from 5 through 32768"),
        }
    }
}

impl From<&AutomaticRecoveryAction> for u16 {
    fn from(value: &AutomaticRecoveryAction) -> Self {
        match value {
            AutomaticRecoveryAction::None => 2,
            AutomaticRecoveryAction::Restart => 3,
            AutomaticRecoveryAction::RevertToSnapshot => 4,
            AutomaticRecoveryAction::DMTF(value) => *value,
        }
    }
}