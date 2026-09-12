use serde::Deserialize;

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

impl<'de> Deserialize<'de> for RecoveryAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}