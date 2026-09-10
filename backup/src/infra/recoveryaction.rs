#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum RecoveryAction {
    Unknown,
    Other,
    DoNotContinue,
    ContinueWithNextJob,
    RerunJob,
    RunRecoveryJob,
}

impl TryFrom<u16> for RecoveryAction {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Other),
            2 => Ok(Self::DoNotContinue),
            3 => Ok(Self::ContinueWithNextJob),
            4 => Ok(Self::RerunJob),
            5 => Ok(Self::RunRecoveryJob),
            _ => Err("Unsupported recovery action"),
        }
    }
}

impl From<RecoveryAction> for u16 {
    fn from(value: RecoveryAction) -> Self {
        match value {
            RecoveryAction::Unknown => 0,
            RecoveryAction::Other => 1,
            RecoveryAction::DoNotContinue => 2,
            RecoveryAction::ContinueWithNextJob => 3,
            RecoveryAction::RerunJob => 4,
            RecoveryAction::RunRecoveryJob => 5,
        }
    }
}