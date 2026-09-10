#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LowMmioGapSize(u64);

impl TryFrom<u64> for LowMmioGapSize {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            128..=3584 => Ok(Self(value)),
            _ => Err("Low MMIO gap size must be between 128 and 3584 MB"),
        }
    }
}

impl From<&LowMmioGapSize> for u64 {
    fn from(value: &LowMmioGapSize) -> Self {
        value.0
    }
}
