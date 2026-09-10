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