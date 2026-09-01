#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnhancedSessionModeState {
    AllowedAndAvailable,
    NotAllowed,
    AllowedButNotAvailable,
}

impl TryFrom<u16> for EnhancedSessionModeState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => Self::AllowedAndAvailable,
            3 => Self::NotAllowed,
            6 => Self::AllowedButNotAvailable,
            _ => return Err("invalid EnhancedSessionModeState value"),
        })
    }
}