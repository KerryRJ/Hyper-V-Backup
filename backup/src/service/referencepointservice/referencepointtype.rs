#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferencePointType {
    LogBased = 1,
    RctBased = 2,
}
