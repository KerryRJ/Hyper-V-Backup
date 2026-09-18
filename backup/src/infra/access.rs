#[derive(Debug, Clone)]
pub(crate) enum Access {
    Unknown,
    Readable,
    Writeable,
    ReadWriteSupported,
    Reserved(u16),
}

impl TryFrom<u16> for Access {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Readable,
            2 => Self::Writeable,
            3 => Self::ReadWriteSupported,
            value => Self::Reserved(value),
        })
    }
}
