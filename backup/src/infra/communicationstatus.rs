use serde::Deserialize;

#[derive(Debug)]
pub(crate) enum CommunicationStatus {
    Unknown,
    NotAvailable,
    CommunicationOk,
    LostCommunication,
    NoContact,
    DMTFReserved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for CommunicationStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommunicationStatus::Unknown),
            1 => Ok(CommunicationStatus::NotAvailable),
            2 => Ok(CommunicationStatus::CommunicationOk),
            3 => Ok(CommunicationStatus::LostCommunication),
            4 => Ok(CommunicationStatus::NoContact),
            5..=32767 => Ok(CommunicationStatus::DMTFReserved(value)),
            32768..=65535 => Ok(CommunicationStatus::VendorReserved(value)),
            _ => Err("Unsupported communication status"),
        }
    }
}

impl<'de> Deserialize<'de> for CommunicationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}