use serde::Serialize;

use crate::model::ReferencePointSettings;

#[derive(Clone, Debug, Serialize)]
#[serde(rename = "INSTANCE")]
pub struct VirtualSystemReferencePointSettingDataIn {
    #[serde(rename = "@CLASSNAME")]
    pub(super) classname: &'static str,
    #[serde(rename = "PROPERTY")]
    pub(super) properties: Vec<super::super::Property>,
}

impl VirtualSystemReferencePointSettingDataIn {
    pub(crate) fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string(self)
    }
}

impl From<&ReferencePointSettings> for VirtualSystemReferencePointSettingDataIn {
    fn from(settings: &ReferencePointSettings) -> Self {
        Self {
            classname: "Msvm_VirtualSystemReferencePointSettingData",
            properties: settings
                .properties
                .iter()
                .map(|property| super::super::Property {
                    name: property.name.clone(),
                    cim_type: property.value.cim_type().to_owned(),
                    value: property.value.value(),
                })
                .collect(),
        }
    }
}
