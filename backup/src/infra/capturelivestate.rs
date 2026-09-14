use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum CaptureLiveState {
    CrashConsistent,       // 0
    Saved,                 // 1
    ApplicationConsistent, // 2
}

impl TryFrom<u8> for CaptureLiveState {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CrashConsistent),
            1 => Ok(Self::Saved),
            2 => Ok(Self::ApplicationConsistent),
            _ => Err("Capture live state must be CrashConsistent (0), Saved (1), or ApplicationConsistent (2)"),
        }
    }
}

impl<'de> Deserialize<'de> for CaptureLiveState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
