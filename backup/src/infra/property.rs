use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct Property {
    #[serde(rename = "@NAME")]
    pub(super) name: &'static str,
    #[serde(rename = "@TYPE")]
    pub(super) cim_type: &'static str,    
    #[serde(rename = "VALUE")]
    pub(super) value: String,
}