#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupProgress {
    pub completed_bytes: u64,
    pub total_bytes: u64,
    pub percent: u8,
}

impl BackupProgress {
    pub(crate) fn new(completed_bytes: u64, total_bytes: u64) -> Self {
        let percent = if total_bytes == 0 { 100 } else { completed_bytes.saturating_mul(100).checked_div(total_bytes).unwrap_or(100).min(100) as u8 };
        Self { completed_bytes, total_bytes, percent }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_bounded_percentages() {
        assert_eq!(BackupProgress::new(0, 100).percent, 0);
        assert_eq!(BackupProgress::new(50, 100).percent, 50);
        assert_eq!(BackupProgress::new(150, 100).percent, 100);
        assert_eq!(BackupProgress::new(0, 0).percent, 100);
    }
}
