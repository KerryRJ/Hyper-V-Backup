use serde::Deserialize;

#[derive(Debug)]
pub(super) enum OperatingStatus {
    Unknown,
    Ok,
    Degraded,
    Error,
    DMTFReserved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for OperatingStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, <Self as TryFrom<u16>>::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Ok),
            2 => Ok(Self::Degraded),
            3 => Ok(Self::Error),
            17..=32767 => Ok(Self::DMTFReserved(value)),
            32768..=65535 => Ok(Self::VendorReserved(value)),
            _ => Err("Unsupported operating status"),
        }
    }
}

impl<'de> Deserialize<'de> for OperatingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
