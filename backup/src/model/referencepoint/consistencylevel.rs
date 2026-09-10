#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsistencyLevel {
    Unknown = 0,
    Application = 1,
    Crash = 2,
}

impl TryFrom<u16> for ConsistencyLevel {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Application),
            2 => Ok(Self::Crash),
            _ => Err(()),
        }
    }
}
