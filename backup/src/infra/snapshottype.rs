pub(super) enum SnapshotType {
    Full,
    Disk,
    DMTF(u16),
    VendorSpecific(u16),
}

impl From<SnapshotType> for u16 {
    fn from(snapshot_type: SnapshotType) -> Self {
        match snapshot_type {
            SnapshotType::Full => 2,
            SnapshotType::Disk => 3,
            SnapshotType::DMTF(value) | SnapshotType::VendorSpecific(value) => value,
        }
    }
}
