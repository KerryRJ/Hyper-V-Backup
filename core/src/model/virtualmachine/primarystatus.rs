#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimaryStatus {
    Unknown,
    Ok,
    Degraded,
    InError,
}

impl TryFrom<u16> for PrimaryStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Ok,
            2 => Self::Degraded,
            3 => Self::InError,
            _ => return Err("invalid PrimaryStatus value"),
        })
    }
}