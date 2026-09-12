use serde::Deserialize;

#[derive(Clone, Debug)]
pub(super) enum PowerManagementCapabilities {
    Unknown,
    NotSupported,
    Disabled,
    Enabled,
    PowerSavingModeEnteredAutomatically,
    PowerStateSettable,
    PowerCycleSupported,
    TimedPowerOnSupported,
}

impl TryFrom<u16> for PowerManagementCapabilities {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::NotSupported),
            2 => Ok(Self::Disabled),
            3 => Ok(Self::Enabled),
            4 => Ok(Self::PowerSavingModeEnteredAutomatically),
            5 => Ok(Self::PowerStateSettable),
            6 => Ok(Self::PowerCycleSupported),
            7 => Ok(Self::TimedPowerOnSupported),
            _ => Err("Unsupported power management capability"),
        }
    }
}

impl<'de> Deserialize<'de> for PowerManagementCapabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
