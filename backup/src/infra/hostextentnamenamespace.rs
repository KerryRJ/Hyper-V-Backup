#[derive(Debug, Clone)]
pub(crate) enum HostExtentNameNamespace {
    Unknown,
    Other,
    Vpd83Type3,
    Vpd83Type2,
    Vpd83Type1,
    Vpd80,
    NodeWwn,
    Snvm,
    OsDeviceNamespace,
    Reserved(u16),
}

impl TryFrom<u16> for HostExtentNameNamespace {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Unknown,
            1 => Self::Other,
            2 => Self::Vpd83Type3,
            3 => Self::Vpd83Type2,
            4 => Self::Vpd83Type1,
            5 => Self::Vpd80,
            6 => Self::NodeWwn,
            7 => Self::Snvm,
            8 => Self::OsDeviceNamespace,
            value => Self::Reserved(value),
        })
    }
}
