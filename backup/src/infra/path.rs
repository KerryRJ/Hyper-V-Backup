#[derive(Clone, Debug)]
pub struct Path(String);

impl Path {
    pub(crate) fn as_str(&self) -> &str {
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
