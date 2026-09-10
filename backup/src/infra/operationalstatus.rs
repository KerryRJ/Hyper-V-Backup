use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum OperationalStatus {
    Unknown,
    Other,
    Ok,
    Degraded,
    Stressed,
    PredictiveFailure,
    Error,
    NonRecoverableError,
    Starting,
    Stopping,
    Stopped,
    InService,
    NoContact,
    LostCommunication,
    Aborted,
    Dormant,
    SupportingEntityInError,
    Completed,
    PowerMode,
}

impl TryFrom<u16> for OperationalStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, &'static str> {
        match value {
            0 => Ok(OperationalStatus::Unknown),
            1 => Ok(OperationalStatus::Other),
            2 => Ok(OperationalStatus::Ok),
            3 => Ok(OperationalStatus::Degraded),
            4 => Ok(OperationalStatus::Stressed),
            5 => Ok(OperationalStatus::PredictiveFailure),
            6 => Ok(OperationalStatus::Error),
            7 => Ok(OperationalStatus::NonRecoverableError),
            8 => Ok(OperationalStatus::Starting),
            9 => Ok(OperationalStatus::Stopping),
            10 => Ok(OperationalStatus::Stopped),
            11 => Ok(OperationalStatus::InService),
            12 => Ok(OperationalStatus::NoContact),
            13 => Ok(OperationalStatus::LostCommunication),
            14 => Ok(OperationalStatus::Aborted),
            15 => Ok(OperationalStatus::Dormant),
            16 => Ok(OperationalStatus::SupportingEntityInError),
            17 => Ok(OperationalStatus::Completed),
            18 => Ok(OperationalStatus::PowerMode),
            _ => Err("Unsupported operational status"),
        }
    }
}

impl<'de> Deserialize<'de> for OperationalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}