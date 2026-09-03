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
}
