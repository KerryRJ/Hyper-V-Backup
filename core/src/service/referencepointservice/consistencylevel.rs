#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsistencyLevel {
    CrashConsistent = 0,
    ApplicationConsistent = 1,
}
