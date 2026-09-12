use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum EnhancedSessionTransportType {
    VMBusPipe, // 0
    HyperVSocket, // 1
}

impl TryFrom<u16> for EnhancedSessionTransportType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::VMBusPipe),
            1 => Ok(Self::HyperVSocket),
            _ => Err("Enhanced session transport type must be VMBusPipe (0) or HyperVSocket (1)"),
        }
    }
}

impl From<&EnhancedSessionTransportType> for u16 {
    fn from(value: &EnhancedSessionTransportType) -> Self {
        match value {
            EnhancedSessionTransportType::VMBusPipe => 0,
            EnhancedSessionTransportType::HyperVSocket => 1,
        }
    }
}