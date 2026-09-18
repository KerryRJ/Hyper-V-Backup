#[derive(Debug, Clone)]
pub(crate) enum WriteHardeningMethod {
    Default,
    WriteCacheEnabled,
    WriteCacheAndFuaEnabled,
    WriteCacheDisabled,
    Reserved(u16),
}

impl TryFrom<u16> for WriteHardeningMethod {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Default,
            1 => Self::WriteCacheEnabled,
            2 => Self::WriteCacheAndFuaEnabled,
            3 => Self::WriteCacheDisabled,
            value => Self::Reserved(value),
        })
    }
}
