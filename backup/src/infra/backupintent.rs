use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum BackupIntent {
    PreserveChain,
    Merge,
}

impl TryFrom<u8> for BackupIntent {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PreserveChain),
            1 => Ok(Self::Merge),
            _ => Err("Backup intent must be PreserveChain (0) or Merge (1)"),
        }
    }
}

impl<'de> Deserialize<'de> for BackupIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
