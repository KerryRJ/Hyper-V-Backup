#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ReturnValue {
    Completed,
    NotSupported,
    Failed,
    Timeout,
    InvalidParameter,
    InvalidState,
    InvalidType,
    MethodParametersCheckedAndJobStarted,
    DMTFReserved(u16),
    MethodReserved(u16),
    VendorSpecific(u16),
}

impl std::fmt::Display for ReturnValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completed => write!(formatter, "Completed with no errors"),
            Self::NotSupported => write!(formatter, "Not supported"),
            Self::Failed => write!(formatter, "Failed"),
            Self::Timeout => write!(formatter, "Timeout"),
            Self::InvalidParameter => write!(formatter, "Invalid parameter"),
            Self::InvalidState => write!(formatter, "Invalid state"),
            Self::InvalidType => write!(formatter, "Invalid type"),
            Self::MethodParametersCheckedAndJobStarted => write!(formatter, "Method parameters checked and job started"),
            Self::DMTFReserved(value) => write!(formatter, "DMTF reserved ({value})"),
            Self::MethodReserved(value) => write!(formatter, "Method reserved ({value})"),
            Self::VendorSpecific(value) => write!(formatter, "Vendor specific ({value})"),
        }
    }
}

impl TryFrom<u16> for ReturnValue {
    type Error = Self;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Completed),
            1 => Err(Self::NotSupported),
            2 => Err(Self::Failed),
            3 => Err(Self::Timeout),
            4 => Err(Self::InvalidParameter),
            5 => Err(Self::InvalidState),
            6 => Err(Self::InvalidType),
            7..=4095 => Err(Self::DMTFReserved(value)),
            4096 => Ok(Self::MethodParametersCheckedAndJobStarted),
            4097..=32767 => Err(Self::MethodReserved(value)),
            32768..=u16::MAX => Err(Self::VendorSpecific(value)),
        }
    }
}
