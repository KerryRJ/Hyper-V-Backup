pub mod communicationstate;
pub mod dedicated;
pub mod detailedstatus;
pub mod enableddefault;
pub mod enabledstate;
pub mod enhancedsessionmodestate;
pub mod healthstate;
pub mod operatingstatus;
pub mod operationalstatus;
pub mod powermanagementcapabilities;
pub mod primarystatus;
pub mod replicationmode;
pub mod requestedstate;
pub mod resetcapability;
#[cfg(test)]
mod tests;
pub mod transitioningtostate;
pub mod virtualmachine;
pub mod vmid;

pub use virtualmachine::VirtualMachine;
pub use vmid::VmId;
