#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePointCreateRequest {
    pub affected_system: String,
    pub reference_point_settings: String,
    pub reference_point_type: u16,
    pub resulting_reference_point: String,
}
