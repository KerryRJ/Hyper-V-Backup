#[derive(Clone, Debug)]
pub(super) enum ReferencePointType {
    Log, // 1
    Rct, //  2
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

impl From<&ReferencePointType> for u16 {
    fn from(value: &ReferencePointType) -> Self {
        match value {
            ReferencePointType::Log => 1,
            ReferencePointType::Rct => 2,
        }
    }
}
