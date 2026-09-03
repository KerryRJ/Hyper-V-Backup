#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WmiPath(String);

impl WmiPath {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for WmiPath {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for WmiPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WmiPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}