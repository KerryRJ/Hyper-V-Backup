#[derive(Clone, Debug)]
pub(super) struct Path(String);

impl Path {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
