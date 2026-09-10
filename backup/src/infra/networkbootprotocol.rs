#[derive(Debug, Clone)]
pub(crate) enum NetworkBootProtocol {
    IPv4,   // 4096
    IPv6,   // 4097
}

impl TryFrom<u16> for NetworkBootProtocol {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            4096 => Ok(Self::IPv4),
            4097 => Ok(Self::IPv6),
            _ => Err("Network boot protocol must be IPv4 (4096) or IPv6 (4097)"),
        }
    }
}

impl From<&NetworkBootProtocol> for u16 {
    fn from(value: &NetworkBootProtocol) -> Self {
        match value {
            NetworkBootProtocol::IPv4 => 4096,
            NetworkBootProtocol::IPv6 => 4097,
        }
    }
}