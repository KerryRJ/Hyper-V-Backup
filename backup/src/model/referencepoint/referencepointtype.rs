#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointType {
    LogBased = 1,
    RctBased = 2,
}

impl TryFrom<u16> for ReferencePointType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::LogBased),
            2 => Ok(Self::RctBased),
            _ => Err(()),
        }
    }
}
