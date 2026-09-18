#[derive(Debug, Clone)]
pub(crate) enum HostExtentNameFormat {
    Unknown,
    Other,
    Snvm,
    Naa,
    Eui64,
    T10Vid,
    OsDeviceName,
    Reserved(u16),
}

impl TryFrom<u16> for HostExtentNameFormat {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Other,
            7 => Self::Snvm,
            9 => Self::Naa,
            10 => Self::Eui64,
            11 => Self::T10Vid,
            12 => Self::OsDeviceName,
            value => Self::Reserved(value),
        })
    }
}
