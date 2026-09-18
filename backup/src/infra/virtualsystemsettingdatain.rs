use serde::Serialize;

use crate::model::SnapshotSettings;

#[derive(Serialize)]
#[serde(rename = "INSTANCE")]
pub(crate) struct VirtualSystemSettingDataIn {
    #[serde(rename = "@CLASSNAME")]
    pub(super) classname: &'static str,
    #[serde(rename = "PROPERTY")]
    pub(super) properties: Vec<super::Property>,
}

impl VirtualSystemSettingDataIn {
    pub(super) fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string(self)
    }
}

impl From<&SnapshotSettings> for VirtualSystemSettingDataIn {
    fn from(settings: &SnapshotSettings) -> Self {
        Self {
            classname: "Msvm_VirtualSystemSnapshotSettingData",
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
