#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommunicationStatus {
    Unknown,
    NotAvailable,
    Ok,
    Lost,
    NoContact,
}

impl TryFrom<u16> for CommunicationStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::NotAvailable,
            2 => Self::Ok,
            3 => Self::Lost,
            4 => Self::NoContact,
            _ => return Err("invalid CommunicationStatus value"),
        })
    }
}