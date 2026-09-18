use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Version {
    major: u16,
    minor: u16,
}

impl TryFrom<String> for Version {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (major, minor) = value.split_once('.').ok_or("Version must use major.minor format")?;

        if major.is_empty() || minor.is_empty() || minor.contains('.') {
            return Err("Version must use major.minor format");
        }

        Ok(Self {
            major: major.parse().map_err(|_| "Version major must be a number")?,
            minor: minor.parse().map_err(|_| "Version minor must be a number")?,
        })
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
