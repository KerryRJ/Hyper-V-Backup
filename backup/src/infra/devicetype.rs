#[derive(Debug, Clone)]
pub(crate) enum DeviceType {
    Floppy,   // 0
    CdRom, // 1
    IdeHardDisk, // 2
    PxeBoot, // 3
    ScsiHardDrive, // 4
    Reserved(u16), // 5
}

impl From<u16> for DeviceType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Floppy,
            1 => Self::CdRom,
            2 => Self::IdeHardDisk,
            3 => Self::PxeBoot,
            4 => Self::ScsiHardDrive,
            5..=u16::MAX => Self::Reserved(value),
        }
    }
}

impl From<&DeviceType> for u16 {
    fn from(value: &DeviceType) -> Self {
        match value {
            DeviceType::Floppy => 0,
            DeviceType::CdRom => 1,
            DeviceType::IdeHardDisk => 2,
            DeviceType::PxeBoot => 3,
            DeviceType::ScsiHardDrive => 4,
            DeviceType::Reserved(value) => *value,
        }
    }
}