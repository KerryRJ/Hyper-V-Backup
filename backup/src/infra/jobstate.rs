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
    QueryPending,
}

impl TryFrom<u16> for JobState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(JobState::New),
            3 => Ok(JobState::Starting),
            4 => Ok(JobState::Running),
            5 => Ok(JobState::Suspended),
            6 => Ok(JobState::ShuttingDown),
            7 => Ok(JobState::Completed),
            8 => Ok(JobState::Terminated),
            9 => Ok(JobState::Killed),
            10 => Ok(JobState::Exception),
            11 => Ok(JobState::Service),
            12 => Ok(JobState::QueryPending),
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
