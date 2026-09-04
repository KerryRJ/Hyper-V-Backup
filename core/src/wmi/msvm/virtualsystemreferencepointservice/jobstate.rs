#[derive(Debug, PartialEq, Eq)]
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
    DMTFReserved(u16),
    VendorReserved(u16),
    Unknown(u16),
}

impl std::fmt::Display for JobState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New => write!(formatter, "New"),
            Self::Starting => write!(formatter, "Starting"),
            Self::Running => write!(formatter, "Running"),
            Self::Suspended => write!(formatter, "Suspended"),
            Self::ShuttingDown => write!(formatter, "Shutting down"),
            Self::Completed => write!(formatter, "Completed"),
            Self::Terminated => write!(formatter, "Terminated"),
            Self::Killed => write!(formatter, "Killed"),
            Self::Exception => write!(formatter, "Exception"),
            Self::Service => write!(formatter, "Service"),
            Self::DMTFReserved(value) => write!(formatter, "DMTF reserved ({value})"),
            Self::VendorReserved(value) => write!(formatter, "Vendor reserved ({value})"),
            Self::Unknown(value) => write!(formatter, "Unknown ({value})"),
        }
    }
}

impl TryFrom<u16> for JobState {
    type Error = Self;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::New),
            3 => Ok(Self::Starting),
            4 => Ok(Self::Running),
            5 => Ok(Self::Suspended),
            6 => Ok(Self::ShuttingDown),
            7 => Ok(Self::Completed),
            8 => Err(Self::Terminated),
            9 => Err(Self::Killed),
            10 => Err(Self::Exception),
            11 => Err(Self::Service),
            12..=32767 => Err(Self::DMTFReserved(value)),
            32768..=u16::MAX => Err(Self::VendorReserved(value)),
            value => Err(Self::Unknown(value)),
        }
    }
}
