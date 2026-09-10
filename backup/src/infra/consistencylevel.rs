#[derive(Clone, Debug)]
pub(super) enum ConsistencyLevel {
    Application, // 1
    Crash,       //  2
}

impl TryFrom<u16> for ConsistencyLevel {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Application),
            2 => Ok(Self::Crash),
            _ => Err("Consistency level must be Application (1) or Crash (2)"),
        }
    }
}

impl From<&ConsistencyLevel> for u16 {
    fn from(value: &ConsistencyLevel) -> Self {
        match value {
            ConsistencyLevel::Application => 1,
            ConsistencyLevel::Crash => 2,
        }
    }
}
