#[derive(Debug, Clone)]
pub(crate) enum CachingMode {
    Unknown,
    Default,
    NoCaching,
    CacheSharableParents,
    Reserved(u16),
}

impl TryFrom<u16> for CachingMode {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            2 => Self::Default,
            3 => Self::NoCaching,
            4 => Self::CacheSharableParents,
            value => Self::Reserved(value),
        })
    }
}
