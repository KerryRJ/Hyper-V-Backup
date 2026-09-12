use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointType {
    Log,
    Rct,
}

impl TryFrom<u16> for ReferencePointType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Log),
            2 => Ok(Self::Rct),
            _ => Err("Reference point type must be Log (1) or Rct (2)"),
        }
    }
}

impl<'de> Deserialize<'de> for ReferencePointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<&ReferencePointType> for u16 {
    fn from(value: &ReferencePointType) -> Self {
        match value {
            ReferencePointType::Log => 1,
            ReferencePointType::Rct => 2,
        }
    }
}
