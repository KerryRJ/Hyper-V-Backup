#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HighMmioGapSize(u64);

impl From<u64> for HighMmioGapSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<&HighMmioGapSize> for u64 {
    fn from(value: &HighMmioGapSize) -> Self {
        value.0
    }
}
