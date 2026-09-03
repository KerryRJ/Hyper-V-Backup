#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointType {
    LogBased = 0,
    RctBased = 1,
}
