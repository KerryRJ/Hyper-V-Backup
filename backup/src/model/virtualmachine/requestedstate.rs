#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestedState {
    Other,
    Enabled,
    Disabled,
    ShutDown,
    NoChange,
    Offline,
    Test,
    Deferred,
    Quiesce,
    Reboot,
    Reset,
    NotApplicable,
    Saving,
    Pausing,
    Resuming,
    FastSaved,
    FastSaving,
    CrticalErrorAndTransientState,
    DmtfReserverved(u16),
    VendorReserved(u16),
}

impl TryFrom<u16> for RequestedState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Other,
            2 => Self::Enabled,
            3 => Self::Disabled,
            4 => Self::ShutDown,
            5 => Self::NoChange,
            6 => Self::Offline,
            7 => Self::Test,
            8 => Self::Deferred,
            9 => Self::Quiesce,
            10 => Self::Reboot,
            11 => Self::Reset,
            12 => Self::NotApplicable,
            13..=32768 => Self::DmtfReserverved(value),
            32773 => Self::Saving,
            32776 => Self::Pausing,
            32777 => Self::Resuming,
            32779 => Self::FastSaved,
            32780 => Self::FastSaving,
            32781..=32792 => Self::CrticalErrorAndTransientState,
            32793..=u16::MAX => Self::VendorReserved(value),
            _ => return Err("invalid RequestedState value"),
        })
    }
}
