use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointTypeIn {
    Log,
    Rct,
}

impl TryFrom<u16> for ReferencePointTypeIn {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Log),
            1 => Ok(Self::Rct),
            _ => Err("Reference point type must be Log (0) or Rct (1)"),
        }
    }
}

impl<'de> Deserialize<'de> for ReferencePointTypeIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<&ReferencePointTypeIn> for u16 {
    fn from(value: &ReferencePointTypeIn) -> Self {
        match value {
            ReferencePointTypeIn::Log => 0,
            ReferencePointTypeIn::Rct => 1,
        }
    }
}
