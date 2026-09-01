#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HealthState {
    Ok,
    MajorFailure,
    CriticalFailure,
}

impl TryFrom<u16> for HealthState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            5 => Self::Ok,
            20 => Self::MajorFailure,
            25 => Self::CriticalFailure,
            _ => return Err("invalid HealthState value"),
        })
    }
}