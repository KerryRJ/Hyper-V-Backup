use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct RunDayOfWeek(i8);

impl TryFrom<i8> for RunDayOfWeek {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -7..=7 => Ok(Self(value)),
            _ => Err("Run day of week must be between -7 and 7"),
        }
    }
}

impl<'de> Deserialize<'de> for RunDayOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(i8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}