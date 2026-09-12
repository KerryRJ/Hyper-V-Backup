use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(super) struct Property {
    #[serde(rename = "@NAME")]
    pub(super) name: String,
    #[serde(rename = "@TYPE")]
    pub(super) cim_type: String,
    #[serde(rename = "VALUE")]
    pub(super) value: String,
}