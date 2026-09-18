use serde::Serialize;

use crate::model::ExportSettings;

#[derive(Serialize)]
#[serde(rename = "INSTANCE")]
pub(crate) struct VirtualSystemExportSettingDataIn {
    #[serde(rename = "@CLASSNAME")]
    pub(super) classname: &'static str,
    #[serde(rename = "PROPERTY")]
    pub(super) properties: Vec<super::Property>,
}

impl VirtualSystemExportSettingDataIn {
    pub(super) fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string(self)
    }
}

impl From<&ExportSettings> for VirtualSystemExportSettingDataIn {
    fn from(settings: &ExportSettings) -> Self {
        Self {
            classname: "Msvm_VirtualSystemExportSettingData",
            properties: settings
                .properties
                .iter()
                .map(|property| super::Property {
                    name: property.name.clone(),
                    cim_type: property.value.cim_type().to_owned(),
                    value: property.value.value(),
                })
                .collect(),
        }
    }
}
