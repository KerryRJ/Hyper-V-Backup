#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalStatus {
    pub primary: PrimaryOperationalStatus,
    pub secondary: Option<SecondaryOperationalStatus>,
}

impl OperationalStatus {
    pub fn from_values(values: Vec<u16>) -> Result<Self, &'static str> {
        let primary = values
            .get(0)
            .ok_or("OperationalStatus index 0 is empty")?
            .to_owned();
        let secondary = values.get(1).map(|v| v.to_owned());
        Ok(Self {
            primary: PrimaryOperationalStatus::try_from(primary)?,
            secondary: match secondary {
                Some(value) => Some(SecondaryOperationalStatus::try_from(value)?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimaryOperationalStatus {
    Ok,
    Degraded,
    PredictiveFailure,
    Stopped,
    InService,
    Dormant,
}

impl TryFrom<u16> for PrimaryOperationalStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => Self::Ok,
            3 => Self::Degraded,
            4 => Self::PredictiveFailure,
            5 => Self::Stopped,
            6 => Self::InService,
            7 => Self::Dormant,
            _ => return Err("invalid OperationalStatus index 0 value"),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecondaryOperationalStatus {
    CreatingSnapshot,
    ApplyingSnapshot,
    DeletingSnapshot,
    WaitingToStart,
    MergingDisks,
    ExportingVirtualMachine,
    MigratingVirtualMachine,
}

impl TryFrom<u16> for SecondaryOperationalStatus {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            32768 => Self::CreatingSnapshot,
            32769 => Self::ApplyingSnapshot,
            32770 => Self::DeletingSnapshot,
            32771 => Self::WaitingToStart,
            32772 => Self::MergingDisks,
            32773 => Self::ExportingVirtualMachine,
            32774 => Self::MigratingVirtualMachine,
            _ => return Err("invalid OperationalStatus index 1 value"),
        })
    }
}