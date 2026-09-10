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
            0 => Ok(PrimaryStatus::Unknown),
            1 => Ok(PrimaryStatus::NotAvailable),
            2 => Ok(PrimaryStatus::Servicing),
            3 => Ok(PrimaryStatus::Starting),
            4 => Ok(PrimaryStatus::Stopping),
            5 => Ok(PrimaryStatus::Stopped),
            6 => Ok(PrimaryStatus::Aborted),
            7 => Ok(PrimaryStatus::Dormant),
            8 => Ok(PrimaryStatus::Completed),
            9 => Ok(PrimaryStatus::Migrating),
            10 => Ok(PrimaryStatus::Emigrating),
            11 => Ok(PrimaryStatus::Immigrating),
            12 => Ok(PrimaryStatus::Snapshotting),
            13 => Ok(PrimaryStatus::ShuttingDown),
            14 => Ok(PrimaryStatus::InTest),
            15 => Ok(PrimaryStatus::Transitioning),
            16 => Ok(PrimaryStatus::InService),
            17..=32767 => Ok(PrimaryStatus::DMTFReserved(value)),
            32768..=65535 => Ok(PrimaryStatus::VendorReserved(value)),
            _ => Err("Unsupported operating status"),
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
