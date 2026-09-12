use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) enum EnabledState { 
    Unknown,
    Other,
    Enabled,
    Disabled,
    ShuttingDown,
    NotApplicable,
    EnabledButOffline,
    InTest,
    Deferred,
    Quiesced,
    Starting,
}

impl TryFrom<u16> for EnabledState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Other),
            2 => Ok(Self::Enabled),
            3 => Ok(Self::Disabled),
            4 => Ok(Self::ShuttingDown),
            5 => Ok(Self::NotApplicable),
            6 => Ok(Self::EnabledButOffline),
            7 => Ok(Self::InTest),
            8 => Ok(Self::Deferred),
            9 => Ok(Self::Quiesced),
            10 => Ok(Self::Starting),
            _ => Err("Unsupported enabled state"),
        }
    }
}

impl<'de> Deserialize<'de> for EnabledState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}