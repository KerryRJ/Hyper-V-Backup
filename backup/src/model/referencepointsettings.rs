#[derive(Clone, Debug, Default)]
pub struct ReferencePointSettings {
    pub properties: Vec<ReferencePointProperty>,
}

#[derive(Clone, Debug)]
pub struct ReferencePointProperty {
    pub name: String,
    pub value: ReferencePointPropertyValue,
}

#[derive(Clone, Debug)]
pub enum ReferencePointPropertyValue {
    Boolean(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    String(String),
}

impl ReferencePointPropertyValue {
    pub(crate) fn cim_type(&self) -> &'static str {
        match self {
            Self::Boolean(_) => "boolean",
            Self::Uint8(_) => "uint8",
            Self::Uint16(_) => "uint16",
            Self::Uint32(_) => "uint32",
            Self::String(_) => "string",
        }
    }

    pub(crate) fn value(&self) -> String {
        match self {
            Self::Boolean(value) => value.to_string(),
            Self::Uint8(value) => value.to_string(),
            Self::Uint16(value) => value.to_string(),
            Self::Uint32(value) => value.to_string(),
            Self::String(value) => value.clone(),
        }
    }
}
