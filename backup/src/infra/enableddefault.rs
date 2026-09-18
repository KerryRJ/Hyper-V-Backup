use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) enum EnabledDefault {
    Enabled,
    Disabled,
    EnabledButOffline,
}

impl TryFrom<u16> for EnabledDefault {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Enabled),
            3 => Ok(Self::Disabled),
            4 => Ok(Self::EnabledButOffline),
            _ => Err("Unsupported enabled default state"),
        }
    }
}

impl<'de> Deserialize<'de> for EnabledDefault {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
