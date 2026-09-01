#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnabledDefault {
    Enabled,
    Disabled,
    EnabledButOffline,
}

impl Default for EnabledDefault {
    fn default() -> Self {
        Self::Enabled
    }
}

impl TryFrom<u16> for EnabledDefault {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => Self::Enabled,
            3 => Self::Disabled,
            6 => Self::EnabledButOffline,
            _ => return Err("invalid EnabledDefault value"),
        })
    }
}