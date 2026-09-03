use crate::model::ConsistencyLevel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointSettingData {
    pub caption: Option<String>,
    pub element_name: Option<String>,
    pub consistency_level: ConsistencyLevel,
}

impl ReferencePointSettingData {
    pub fn new(consistency_level: ConsistencyLevel) -> Self {
        Self {
            caption: None,
            element_name: None,
            consistency_level,
        }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<INSTANCE CLASSNAME=\"Msvm_VirtualSystemReferencePointSettingData\">");
        if let Some(ref name) = self.caption {
            xml.push_str(&format!("<PROPERTY NAME=\"Caption\" TYPE=\"string\"><VALUE>{}</VALUE></PROPERTY>", name));
        }
        if let Some(ref name) = self.element_name {
            xml.push_str(&format!("<PROPERTY NAME=\"ElementName\" TYPE=\"string\"><VALUE>{}</VALUE></PROPERTY>", name));
        }
        xml.push_str(&format!(
            "<PROPERTY NAME=\"ConsistencyLevel\" TYPE=\"uint8\"><VALUE>{}</VALUE></PROPERTY>",
            self.consistency_level as u8
        ));
        xml.push_str("</INSTANCE>");
        xml
    }
}
