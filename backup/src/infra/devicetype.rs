use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum DeviceType {
    Floppy,        // 0
    CdRom,         // 1
    IdeHardDisk,   // 2
    PxeBoot,       // 3
    ScsiHardDrive, // 4
    Reserved(u16), // 5
}

impl TryFrom<u16> for DeviceType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Floppy),
            1 => Ok(Self::CdRom),
            2 => Ok(Self::IdeHardDisk),
            3 => Ok(Self::PxeBoot),
            4 => Ok(Self::ScsiHardDrive),
            5..=u16::MAX => Ok(Self::Reserved(value)),
            _ => Err("Unsupported device type"),
        }
    }
}

impl<'de> Deserialize<'de> for DeviceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
