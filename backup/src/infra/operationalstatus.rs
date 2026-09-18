use serde::Deserialize;

#[derive(Clone, Debug)]
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
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Other),
            2 => Ok(Self::Ok),
            3 => Ok(Self::Degraded),
            4 => Ok(Self::Stressed),
            5 => Ok(Self::PredictiveFailure),
            6 => Ok(Self::Error),
            7 => Ok(Self::NonRecoverableError),
            8 => Ok(Self::Starting),
            9 => Ok(Self::Stopping),
            10 => Ok(Self::Stopped),
            11 => Ok(Self::InService),
            12 => Ok(Self::NoContact),
            13 => Ok(Self::LostCommunication),
            14 => Ok(Self::Aborted),
            15 => Ok(Self::Dormant),
            16 => Ok(Self::SupportingEntityInError),
            17 => Ok(Self::Completed),
            18 => Ok(Self::PowerMode),
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
