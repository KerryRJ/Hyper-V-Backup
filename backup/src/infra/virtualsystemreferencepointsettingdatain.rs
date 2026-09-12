use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "INSTANCE")]
pub struct VirtualSystemReferencePointSettingDataIn {
    #[serde(rename = "@CLASSNAME")]
    pub(super) classname: &'static str,    
    #[serde(rename = "PROPERTY")]
    pub(super) properties: Vec<super::Property>,
}

impl VirtualSystemReferencePointSettingDataIn {
    pub(super) fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string(self)
    }
}