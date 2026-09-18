#[derive(Debug, Clone)]
pub(crate) enum AutomaticCriticalErrorAction {
    None,        // 0
    PauseResume, // 1
}

impl From<u16> for AutomaticCriticalErrorAction {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,
            1 => Self::PauseResume,
            value => panic!("Unsupported value for AutomaticCriticalErrorAction: {}", value),
        }
    }
}

impl From<&AutomaticCriticalErrorAction> for u16 {
    fn from(value: &AutomaticCriticalErrorAction) -> Self {
        match value {
            AutomaticCriticalErrorAction::None => 0,
            AutomaticCriticalErrorAction::PauseResume => 1,
        }
    }
}
