#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Dedicated {
    Not,
    Unknown,
    Other,
}

impl TryFrom<u16> for Dedicated {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Not,
            1 => Self::Unknown,
            2 => Self::Other,
            _ => return Err("invalid Dedicated value"),
        })
    }
}