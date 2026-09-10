use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Percent(u16);

impl TryFrom<u16> for Percent {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0..=100 => Ok(Self(value)),
            _ => Err("Percent complete must be between 0 and 100"),
        }
    }
}

impl From<Percent> for u16 {
    fn from(value: Percent) -> Self {
        value.0
    }
}

impl Display for Percent {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}