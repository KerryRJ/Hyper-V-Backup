#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct VirtualDiskRange {
    pub(crate) byte_offset: u64,
    pub(crate) byte_length: u64,
}
