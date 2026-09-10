#[derive(Debug, Clone)]
pub(crate) enum ConsoleMode {
    Default, // 0
    Com1, // 1
    Com2, // 2
    None, // 3
}

impl TryFrom<u16> for ConsoleMode {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Com1),
            2 => Ok(Self::Com2),
            3 => Ok(Self::None),
            _ => Err("Console mode must be Default (0), Com1 (1), Com2 (2), or None (3)"),
        }
    }
}

impl From<&ConsoleMode> for u16 {
    fn from(value: &ConsoleMode) -> Self {
        match value {
            ConsoleMode::Default => 0,
            ConsoleMode::Com1 => 1,
            ConsoleMode::Com2 => 2,
            ConsoleMode::None => 3,
        }
    }
}