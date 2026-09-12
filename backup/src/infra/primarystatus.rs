use serde::Deserialize;

#[derive(Debug)]
pub(super) enum PrimaryStatus {
    Unknown,
    NotAvailable,
    Servicing,
    Starting,
    Stopping,
    Stopped,
    Aborted,
    Dormant,
    Completed,
    Migrating,
    Emigrating,
    Immigrating,
    Snapshotting,
    ShuttingDown,
    InTest,
    Transitioning,
    InService,
    DMTFReserved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for PrimaryStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::NotAvailable),
            2 => Ok(Self::Servicing),
            3 => Ok(Self::Starting),
            4 => Ok(Self::Stopping),
            5 => Ok(Self::Stopped),
            6 => Ok(Self::Aborted),
            7 => Ok(Self::Dormant),
            8 => Ok(Self::Completed),
            9 => Ok(Self::Migrating),
            10 => Ok(Self::Emigrating),
            11 => Ok(Self::Immigrating),
            12 => Ok(Self::Snapshotting),
            13 => Ok(Self::ShuttingDown),
            14 => Ok(Self::InTest),
            15 => Ok(Self::Transitioning),
            16 => Ok(Self::InService),
            17..=32767 => Ok(Self::DMTFReserved(value)),
            32768..=65535 => Ok(Self::VendorReserved(value)),
            _ => Err("Unsupported primary status"),
        }
    }
}

impl<'de> Deserialize<'de> for PrimaryStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
