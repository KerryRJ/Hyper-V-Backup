use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum Format {
    Vhd,
    Vhdx,
    VhdSet,
    Reserved(u16),
}

impl TryFrom<u16> for Format {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Vhd),
            3 => Ok(Self::Vhdx),
            4 => Ok(Self::VhdSet),
            _ => Err("Format"),
        }
    }
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
