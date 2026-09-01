use uuid::Uuid;

pub type ReferencePointId = Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencePoint {
    id: ReferencePointId,
}

impl ReferencePoint {
    pub(crate) fn new(id: ReferencePointId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> ReferencePointId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::ReferencePoint;
    use uuid::Uuid;

    #[test]
    fn exposes_wmi_reference_identifier() {
        let reference_point = ReferencePoint::new(Uuid::nil());

        assert_eq!(reference_point.id(), Uuid::nil());
    }
}
