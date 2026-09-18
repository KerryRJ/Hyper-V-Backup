use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum NetworkBootProtocol {
    IPv4, // 4096
    IPv6, // 4097
}

impl TryFrom<u16> for NetworkBootProtocol {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            4096 => Ok(Self::IPv4),
            4097 => Ok(Self::IPv6),
            _ => Err("Network boot protocol must be IPv4 (4096) or IPv6 (4097)"),
        }
    }
}

impl<'de> Deserialize<'de> for NetworkBootProtocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
