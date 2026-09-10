#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperatingStatus {
    Unknown,
    NotAvailable,
    InService,
    Starting,
    Stopping,
    Stopped,
    Aborted,
    Dormant,
    Completed,
    Migrating,
    InTest,
    Snapshotting,
}

impl TryFrom<u16> for OperatingStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::NotAvailable,
            2 => Self::InService,
            3 => Self::Starting,
            4 => Self::Stopping,
            5 => Self::Stopped,
            6 => Self::Aborted,
            7 => Self::Dormant,
            8 => Self::Completed,
            9 => Self::Migrating,
            10 => Self::InTest,
            11 => Self::Snapshotting,
            _ => return Err("invalid OperatingStatus value"),
        })
    }
}
