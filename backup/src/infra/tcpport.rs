#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TcpPort(u16);

impl TryFrom<u32> for TcpPort {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        u16::try_from(value)
            .map(Self)
            .map_err(|_| "TCP port must be between 0 and 65535")
    }
}

impl From<&TcpPort> for u32 {
    fn from(value: &TcpPort) -> Self {
        u32::from(value.0)
    }
}
