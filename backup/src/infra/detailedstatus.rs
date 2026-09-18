use serde::Deserialize;

#[derive(Clone, Debug)]
pub(crate) enum DetailedStatus {
    NotAvailable,
    NoAdditionalInformation,
    Stressed,
    PredictiveFailure,
    NonRecoverableError,
    SupportingEntityInError,
    DMTFReserved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for DetailedStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotAvailable),
            1 => Ok(Self::NoAdditionalInformation),
            2 => Ok(Self::Stressed),
            3 => Ok(Self::PredictiveFailure),
            4 => Ok(Self::NonRecoverableError),
            5 => Ok(Self::SupportingEntityInError),
            6..=32767 => Ok(Self::DMTFReserved(value)),
            32768..=65535 => Ok(Self::VendorReserved(value)),
            _ => Err("Unsupported detailed status"),
        }
    }
}

impl<'de> Deserialize<'de> for DetailedStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
