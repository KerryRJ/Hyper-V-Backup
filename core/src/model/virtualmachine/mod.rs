pub mod communicationstate;
pub mod dedicated;
pub mod detailedstatus;
pub mod enableddefault;
pub mod enabledstate;
pub mod enhancedsessionmodestate;
pub mod healthstate;
pub mod vmid;
pub mod operatingstatus;
pub mod operationalstatus;
pub mod powermanagementcapabilities;
pub mod primarystatus;
pub mod replicationmode;
pub mod requestedstate;
pub mod resetcapability;
pub mod transitioningtostate;
pub mod virtualmachine;
#[cfg(test)]
mod tests;

pub use vmid::VmId;
pub use virtualmachine::VirtualMachine;
