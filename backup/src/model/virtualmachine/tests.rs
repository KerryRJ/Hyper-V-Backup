use std::time::Duration;

use crate::model::Error;
use crate::model::VmId;
use crate::infra::computersystemout::ComputerSystemOut;

use super::{
    VirtualMachine,
    communicationstate::CommunicationStatus,
    dedicated::Dedicated,
    detailedstatus::DetailedStatus,
    enableddefault::EnabledDefault,
    enabledstate::EnabledState,
    enhancedsessionmodestate::EnhancedSessionModeState,
    healthstate::HealthState,
    operatingstatus::OperatingStatus,
    operationalstatus::{OperationalStatus, PrimaryOperationalStatus, SecondaryOperationalStatus},
    powermanagementcapabilities::PowerManagementCapabilities,
    primarystatus::PrimaryStatus,
    replicationmode::ReplicationMode,
    requestedstate::RequestedState,
    resetcapability::ResetCapability,
    transitioningtostate::TransitioningToState,
};

#[test]
fn converts_minimal_computer_system() {
    let mut computer_system = ComputerSystemOut::default();
    computer_system.Name = "11111111-1111-1111-1111-111111111111".into();

    let machine = VirtualMachine::try_from(computer_system).unwrap();

    assert_eq!(machine.id(), VmId::parse_str("11111111-1111-1111-1111-111111111111").unwrap());
    assert_eq!(machine.caption(), None);
    assert_eq!(machine.on_time(), None);
}

#[test]
fn converts_basic_computer_system_fields() {
    let mut computer_system = ComputerSystemOut::default();
    computer_system.Name = "11111111-1111-1111-1111-111111111111".into();
    computer_system.Caption = Some("Virtual Machine".into());
    computer_system.ElementName = Some("vm01".into());
    computer_system.OnTimeInMilliseconds = Some(2_500);
    computer_system.EnabledState = Some(2);
    computer_system.HealthState = Some(5);

    let machine = VirtualMachine::try_from(computer_system).unwrap();

    assert_eq!(machine.name(), Some("vm01"));
    assert_eq!(machine.caption(), Some("Virtual Machine"));
    assert_eq!(machine.on_time(), Some(Duration::from_millis(2_500)));
    assert_eq!(machine.enabled_state(), Some(&EnabledState::Enabled));
    assert_eq!(machine.health_state(), Some(&HealthState::Ok));
}

#[test]
fn rejects_invalid_virtual_machine_id() {
    let mut computer_system = ComputerSystemOut::default();
    computer_system.Name = "not-a-uuid".into();

    assert!(matches!(VirtualMachine::try_from(computer_system), Err(Error::Uuid(_))));
}

#[test]
fn rejects_invalid_optional_field() {
    let mut computer_system = ComputerSystemOut::default();
    computer_system.Name = uuid::Uuid::nil().to_string();
    computer_system.CommunicationStatus = Some(99);

    assert!(matches!(VirtualMachine::try_from(computer_system), Err(Error::InvalidField("CommunicationStatus"))));
}

#[test]
fn decodes_non_sequential_status_values() {
    assert_eq!(CommunicationStatus::try_from(2), Ok(CommunicationStatus::Ok));
    assert_eq!(EnabledState::try_from(32768), Ok(EnabledState::Paused));
    assert_eq!(HealthState::try_from(25), Ok(HealthState::CriticalFailure));
    assert_eq!(OperatingStatus::try_from(11), Ok(OperatingStatus::Snapshotting));
    assert_eq!(RequestedState::try_from(32780), Ok(RequestedState::FastSaving));
    assert_eq!(TransitioningToState::try_from(32777), Ok(TransitioningToState::Resuming));
}

#[test]
fn rejects_unknown_status_values() {
    assert!(CommunicationStatus::try_from(99).is_err());
    assert!(EnabledState::try_from(99).is_err());
    assert!(HealthState::try_from(99).is_err());
    assert!(OperatingStatus::try_from(99).is_err());
    assert!(RequestedState::try_from(0).is_err());
    assert!(TransitioningToState::try_from(1).is_err());
}

#[test]
fn decodes_operational_status_primary_and_secondary_values() {
    let status = OperationalStatus::from_values(vec![2, 32773]).unwrap();

    assert_eq!(status.primary, PrimaryOperationalStatus::Ok);
    assert_eq!(status.secondary, Some(SecondaryOperationalStatus::ExportingVirtualMachine));
}

#[test]
fn rejects_empty_or_invalid_operational_status() {
    assert!(OperationalStatus::from_values(vec![]).is_err());
    assert!(OperationalStatus::from_values(vec![2, 99]).is_err());
}

#[test]
fn decodes_remaining_status_values() {
    assert_eq!(Dedicated::try_from(2), Ok(Dedicated::Other));
    assert_eq!(DetailedStatus::try_from(4), Ok(DetailedStatus::NonRecoverableError));
    assert_eq!(EnabledDefault::default(), EnabledDefault::Enabled);
    assert_eq!(EnabledDefault::try_from(6), Ok(EnabledDefault::EnabledButOffline));
    assert_eq!(EnhancedSessionModeState::try_from(6), Ok(EnhancedSessionModeState::AllowedButNotAvailable));
    assert_eq!(PowerManagementCapabilities::try_from(7), Ok(PowerManagementCapabilities::TimedPowerOnSupported));
    assert_eq!(PrimaryStatus::try_from(3), Ok(PrimaryStatus::InError));
    assert_eq!(ReplicationMode::try_from(4), Ok(ReplicationMode::ExtendedReplica));
    assert_eq!(ResetCapability::try_from(5), Ok(ResetCapability::NotImplemented));
}

#[test]
fn rejects_remaining_unknown_status_values() {
    assert!(Dedicated::try_from(99).is_err());
    assert!(DetailedStatus::try_from(99).is_err());
    assert!(EnabledDefault::try_from(99).is_err());
    assert!(EnhancedSessionModeState::try_from(99).is_err());
    assert!(PowerManagementCapabilities::try_from(99).is_err());
    assert!(PrimaryStatus::try_from(99).is_err());
    assert!(ReplicationMode::try_from(99).is_err());
    assert!(ResetCapability::try_from(99).is_err());
}
