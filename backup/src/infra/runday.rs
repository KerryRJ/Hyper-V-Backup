#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct RunDay(i8);

impl TryFrom<i8> for RunDay {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -31..=31 => Ok(Self(value)),
            _ => Err("Run day must be between -31 and 31"),
        }
    }
}