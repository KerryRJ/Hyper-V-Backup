use serde::Deserialize;

#[derive(Debug)]
pub(super) enum JobState {
    New,
    Starting,
    Running,
    Suspended,
    ShuttingDown,
    Completed,
    Terminated,
    Killed,
    Exception,
    Service,
    // QueryPending,
    DmtfReserved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for JobState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::New),
            3 => Ok(Self::Starting),
            4 => Ok(Self::Running),
            5 => Ok(Self::Suspended),
            6 => Ok(Self::ShuttingDown),
            7 => Ok(Self::Completed),
            8 => Ok(Self::Terminated),
            9 => Ok(Self::Killed),
            10 => Ok(Self::Exception),
            11 => Ok(Self::Service),
            // 12 => Ok(JobState::QueryPending),
            11..=32767 => Ok(Self::DmtfReserved(value)),
            32768..=65535 => Ok(Self::VendorReserved(value)),
            _ => Err("Unsupported job state"),
        }
    }
}

impl<'de> Deserialize<'de> for JobState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
