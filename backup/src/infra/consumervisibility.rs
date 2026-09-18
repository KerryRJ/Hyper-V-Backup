#[derive(Debug, Clone)]
pub(crate) enum ConsumerVisibility {
    Unknown,
    PassedThrough,
    Virtualized,
    NotRepresented,
    Reserved(u16),
}

impl TryFrom<u16> for ConsumerVisibility {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            2 => Self::PassedThrough,
            3 => Self::Virtualized,
            4 => Self::NotRepresented,
            value => Self::Reserved(value),
        })
    }
}
