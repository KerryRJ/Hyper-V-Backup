#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DetailedStatus {
    NotAvailable,
    NoAdditionalInformation,
    Stressed,
    PredictiveFailure,
    NonRecoverableError,
}

impl TryFrom<u16> for DetailedStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::NotAvailable,
            1 => Self::NoAdditionalInformation,
            2 => Self::Stressed,
            3 => Self::PredictiveFailure,
            4 => Self::NonRecoverableError,
            _ => return Err("invalid DetailedStatus value"),
        })
    }
}
