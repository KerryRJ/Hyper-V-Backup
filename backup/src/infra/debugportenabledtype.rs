#[derive(Debug, Clone)]
pub(crate) enum DebugPortEnabledType {
    Off, // 0
    On, // 1
    OnAutoAssigned, // 2
}

impl TryFrom<u16> for DebugPortEnabledType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            2 => Ok(Self::OnAutoAssigned),
            _ => Err("Debug port enabled type must be Off (0), On (1), or OnAutoAssigned (2)"),
        }
    }
}

impl From<&DebugPortEnabledType> for u16 {
    fn from(value: &DebugPortEnabledType) -> Self {
        match value {
            DebugPortEnabledType::Off => 0,
            DebugPortEnabledType::On => 1,
            DebugPortEnabledType::OnAutoAssigned => 2,
        }
    }
}