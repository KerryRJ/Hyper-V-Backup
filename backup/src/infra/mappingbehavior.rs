#[derive(Debug, Clone)]
pub(crate) enum MappingBehavior {
    Unknown,
    NotSupported,
    Dedicated,
    SoftAffinity,
    HardAffinity,
    Reserved(u16),
}

impl TryFrom<u16> for MappingBehavior {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::NotSupported,
            2 => Self::Dedicated,
            3 => Self::SoftAffinity,
            4 => Self::HardAffinity,
            value => Self::Reserved(value),
        })
    }
}
