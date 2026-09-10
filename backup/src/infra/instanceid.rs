use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct InstanceId(String);

impl InstanceId {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for InstanceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for InstanceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
