#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PowerManagementCapabilities {
    Unknown,
    NotSupported,
    Disabled,
    Enabled,
    PowerSavingModesEnabledAutomatically,
    PowerStateSettable,
    PowerCyclingSupported,
    TimedPowerOnSupported,
}

impl TryFrom<u16> for PowerManagementCapabilities {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::NotSupported,
            2 => Self::Disabled,
            3 => Self::Enabled,
            4 => Self::PowerSavingModesEnabledAutomatically,
            5 => Self::PowerStateSettable,
            6 => Self::PowerCyclingSupported,
            7 => Self::TimedPowerOnSupported,
            _ => return Err("invalid PowerManagementCapabilities value"),
        })
    }
}