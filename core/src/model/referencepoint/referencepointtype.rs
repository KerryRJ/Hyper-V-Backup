#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointType {
    LogBased = 0,
    RctBased = 1,
}

impl TryFrom<u16> for ReferencePointType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LogBased),
            1 => Ok(Self::RctBased),
            _ => Err(()),
        }
    }
}