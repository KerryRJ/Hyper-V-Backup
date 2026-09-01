#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResetCapability {
    Unknown,
    Other,
    None,
    Disabled,
    Enabled,
    NotImplemented,
}

impl TryFrom<u16> for ResetCapability {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Other,
            2 => Self::None,
            3 => Self::Disabled,
            4 => Self::Enabled,
            5 => Self::NotImplemented,
            _ => return Err("invalid ResetCapability value"),
        })
    }
}