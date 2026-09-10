#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplicationMode {
    None,
    Primary,
    Replica,
    TestReplica,
    ExtendedReplica,
}

impl TryFrom<u16> for ReplicationMode {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::Primary,
            2 => Self::Replica,
            3 => Self::TestReplica,
            4 => Self::ExtendedReplica,
            _ => return Err("invalid ReplicationMode value"),
        })
    }
}
