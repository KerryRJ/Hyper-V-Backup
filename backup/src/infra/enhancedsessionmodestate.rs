use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) enum EnhancedSessionModeState {
    AllowedAndAvailable,
    NotAllowed,
    AllowedButNotAvailable,
}

impl TryFrom<u16> for EnhancedSessionModeState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::AllowedAndAvailable),
            3 => Ok(Self::NotAllowed),
            6 => Ok(Self::AllowedButNotAvailable),
            _ => Err("Unsupported enhanced session mode state"),
        }
    }
}

impl<'de> Deserialize<'de> for EnhancedSessionModeState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
