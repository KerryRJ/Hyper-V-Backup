use super::consistencylevel::ConsistencyLevel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointSettingsData {
    pub element_name: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub consistency_level: Option<ConsistencyLevel>,
}

impl ReferencePointSettingsData {
    pub fn new() -> Self {
        Self {
            element_name: None,
            caption: None,
            description: None,
            consistency_level: None,
        }
    }

    pub fn to_xml(&self) -> String {
        let consistency_level = self
            .consistency_level
            .map(|value| format!("<PROPERTY NAME=\"ConsistencyLevel\" TYPE=\"uint8\"><VALUE>{}</VALUE></PROPERTY>", value as u8))
            .unwrap_or_default();
        let caption = self.caption.as_deref().map(|value| format!("<PROPERTY NAME=\"Caption\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>")).unwrap_or_default();
        let description = self
            .description
            .as_deref()
            .map(|value| format!("<PROPERTY NAME=\"Description\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>"))
            .unwrap_or_default();
        let element_name = self
            .element_name
            .as_deref()
            .map(|value| format!("<PROPERTY NAME=\"ElementName\" TYPE=\"string\"><VALUE>{value}</VALUE></PROPERTY>"))
            .unwrap_or_default();

        format!("<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\">{consistency_level}{caption}{description}{element_name}</INSTANCE>")
    }
}
