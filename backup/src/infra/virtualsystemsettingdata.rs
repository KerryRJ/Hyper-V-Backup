use crate::infra::*;
use serde::{Deserialize, Deserializer};
use std::{fmt, path::PathBuf};
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct VirtualSystemSettingData {
    pub(crate) path: Path,                                                 // r
    additional_recovery_information: Option<String>,                       // rw
    allow_full_scsi_command_set: Option<bool>,                             // rw
    allow_reduced_fc_redundancy: bool,                                     // rw
    architecture: String,                                                  // r
    automatic_critical_error_action: Option<AutomaticCriticalErrorAction>, // rw
    automatic_critical_error_action_timeout: Option<chrono::Duration>,     // rw
    automatic_recovery_action: Option<AutomaticRecoveryAction>,            // r
    automatic_shutdown_action: Option<AutomaticShutdownAction>,            // r
    automatic_snapshots_enabled: bool,                                     //rw
    automatic_startup_action: Option<AutomaticStartupAction>,              // r
    automatic_startup_action_delay: Option<chrono::Duration>,              // r
    automatic_startup_action_sequence_number: Option<SequenceNumber>,      // r
    base_board_serial_number: String,                                      // rw
    bios_guid: String,                                                     // rw
    bios_num_lock: bool,                                                   // rw
    bios_serial_number: String,                                            // rw
    boot_order: Vec<DeviceType>,                                           // rw
    boot_source_order: Vec<String>,                                        // rw
    caption: String,                                                       // r
    chassis_asset_tag: String,                                             // rw
    chassis_serial_number: String,                                         // rw
    configuration_data_root: PathBuf,                                      // r
    configuration_file: PathBuf,                                           // r
    configuration_id: String,                                              // Uuid // r
    console_mode: ConsoleMode,                                             // rw
    creation_time: chrono::DateTime<chrono::Utc>,                          // r
    debug_channel_id: Option<u32>,                                         // rw
    debug_port: Option<TcpPort>,                                           // rw
    debug_port_enabled: Option<DebugPortEnabledType>,                      // rw
    description: String,                                                   // r // "Active settings for the virtual machine" or "Snapshot settings for the virtual machine"
    element_name: String,                                                  // r
    enhanced_session_transport_type: EnhancedSessionTransportType,         // rw
    guest_controlled_cache_types: bool,                                    // rw
    guest_state_data_root: PathBuf,                                        // r
    guest_state_file: PathBuf,                                             // r
    high_mmio_gap_size: HighMmioGapSize,                                   // rw
    incremental_backup_enabled: Option<bool>,                              // rw
    instance_id: InstanceId,                                               // r
    is_automatic_snapshot: bool,                                           // r
    is_saved: bool,                                                        // r
    lock_on_disconnect: bool,                                              // rw
    log_data_root: Option<PathBuf>,                                        // r
    low_mmio_gap_size: LowMmioGapSize,                                     // rw
    network_boot_preferred_protocol: NetworkBootProtocol,                  // rw
    notes: Vec<String>,                                                    // r
    parent: Option<String>,                                                // r
    parent_package: Option<String>,                                        // rw
    pause_after_boot_failure: bool,                                        // rw
    recovery_file: Option<PathBuf>,                                        // r
    secure_boot_enabled: bool,                                             // rw
    secure_boot_template_id: String,                                       // rw
    snapshot_data_root: Option<PathBuf>,                                   // r
    suspend_data_root: PathBuf,                                            // r
    swap_file_data_root: Option<PathBuf>,                                  // r
    user_snapshot_type: UserSnapshotType,                                  // rw
    version: Version,                                                      // r
    virtual_numa_enabled: bool,                                            // r
    virtual_system_identifier: VirtualMachineId,                           // r
    virtual_system_sub_type: String,                                       // r
    virtual_system_type: String,                                           // r
}

impl fmt::Debug for VirtualSystemSettingData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VirtualSystemSettingData")
            .field("path", &self.path)
            .field("additional_recovery_information", &self.additional_recovery_information)
            .field("allow_full_scsi_command_set", &self.allow_full_scsi_command_set)
            .field("allow_reduced_fc_redundancy", &self.allow_reduced_fc_redundancy)
            .field("architecture", &self.architecture)
            .field("automatic_critical_error_action", &self.automatic_critical_error_action)
            .field("automatic_critical_error_action_timeout", &self.automatic_critical_error_action_timeout)
            .field("automatic_recovery_action", &self.automatic_recovery_action)
            .field("automatic_shutdown_action", &self.automatic_shutdown_action)
            .field("automatic_snapshots_enabled", &self.automatic_snapshots_enabled)
            .field("automatic_startup_action", &self.automatic_startup_action)
            .field("automatic_startup_action_delay", &self.automatic_startup_action_delay)
            .field("automatic_startup_action_sequence_number", &self.automatic_startup_action_sequence_number)
            .field("base_board_serial_number", &self.base_board_serial_number)
            .field("bios_guid", &self.bios_guid)
            .field("bios_num_lock", &self.bios_num_lock)
            .field("bios_serial_number", &self.bios_serial_number)
            .field("boot_order", &self.boot_order)
            .field("boot_source_order", &self.boot_source_order)
            .field("caption", &self.caption)
            .field("chassis_asset_tag", &self.chassis_asset_tag)
            .field("chassis_serial_number", &self.chassis_serial_number)
            .field("configuration_data_root", &self.configuration_data_root)
            .field("configuration_file", &self.configuration_file)
            .field("configuration_id", &self.configuration_id)
            .field("console_mode", &self.console_mode)
            .field("creation_time", &self.creation_time)
            .field("debug_channel_id", &self.debug_channel_id)
            .field("debug_port", &self.debug_port)
            .field("debug_port_enabled", &self.debug_port_enabled)
            .field("description", &self.description)
            .field("element_name", &self.element_name)
            .field("enhanced_session_transport_type", &self.enhanced_session_transport_type)
            .field("guest_controlled_cache_types", &self.guest_controlled_cache_types)
            .field("guest_state_data_root", &self.guest_state_data_root)
            .field("guest_state_file", &self.guest_state_file)
            .field("high_mmio_gap_size", &self.high_mmio_gap_size)
            .field("incremental_backup_enabled", &self.incremental_backup_enabled)
            .field("instance_id", &self.instance_id)
            .field("is_automatic_snapshot", &self.is_automatic_snapshot)
            .field("is_saved", &self.is_saved)
            .field("lock_on_disconnect", &self.lock_on_disconnect)
            .field("log_data_root", &self.log_data_root)
            .field("low_mmio_gap_size", &self.low_mmio_gap_size)
            .field("network_boot_preferred_protocol", &self.network_boot_preferred_protocol)
            .field("notes", &self.notes)
            .field("parent", &self.parent)
            .field("parent_package", &self.parent_package)
            .field("pause_after_boot_failure", &self.pause_after_boot_failure)
            .field("recovery_file", &self.recovery_file)
            .field("secure_boot_enabled", &self.secure_boot_enabled)
            .field("secure_boot_template_id", &self.secure_boot_template_id)
            .field("snapshot_data_root", &self.snapshot_data_root)
            .field("suspend_data_root", &self.suspend_data_root)
            .field("swap_file_data_root", &self.swap_file_data_root)
            .field("user_snapshot_type", &self.user_snapshot_type)
            .field("version", &self.version)
            .field("virtual_numa_enabled", &self.virtual_numa_enabled)
            .field("virtual_system_identifier", &self.virtual_system_identifier)
            .field("virtual_system_sub_type", &self.virtual_system_sub_type)
            .field("virtual_system_type", &self.virtual_system_type)
            .finish()
    }
}

impl VirtualSystemSettingData {
    pub(crate) fn is_snapshot(&self) -> bool {
        self.description == "Snapshot settings for the virtual machine"
    }

    pub(super) fn instance_id(&self) -> &str {
        self.instance_id.as_ref()
    }
}

impl<'de> Deserialize<'de> for VirtualSystemSettingData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let output = super::VirtualSystemSettingDataOut::deserialize(deserializer)?;
        let automatic_startup_action_delay = output.AutomaticStartupActionDelay.map(|value| chrono::Duration::from_std(value.0)).transpose().map_err(serde::de::Error::custom)?;
        let automatic_critical_error_action_timeout = output.AutomaticCriticalErrorActionTimeout.map(|value| chrono::Duration::from_std(value.0)).transpose().map_err(serde::de::Error::custom)?;
        let virtual_system_identifier = Uuid::parse_str(&output.VirtualSystemIdentifier).map_err(serde::de::Error::custom)?;
        if virtual_system_identifier.is_nil() {
            return Err(serde::de::Error::custom("VirtualSystemIdentifier cannot be nil"));
        }
        let virtual_system_identifier = super::VirtualMachineId::new(virtual_system_identifier).expect("VirtualSystemIdentifier was already checked for nil");

        Ok(Self {
            path: output.__Path.into(),
            additional_recovery_information: output.AdditionalRecoveryInformation,
            allow_full_scsi_command_set: output.AllowFullSCSICommandSet,
            allow_reduced_fc_redundancy: output.AllowReducedFcRedundancy,
            architecture: output.Architecture,
            automatic_critical_error_action: output.AutomaticCriticalErrorAction.map(Into::into),
            automatic_critical_error_action_timeout,
            automatic_recovery_action: output.AutomaticRecoveryAction.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            automatic_shutdown_action: output.AutomaticShutdownAction.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            automatic_snapshots_enabled: output.AutomaticSnapshotsEnabled,
            automatic_startup_action: output.AutomaticStartupAction.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            automatic_startup_action_delay,
            automatic_startup_action_sequence_number: output.AutomaticStartupActionSequenceNumber.map(Into::into),
            base_board_serial_number: output.BaseBoardSerialNumber,
            bios_guid: output.BIOSGUID,
            bios_num_lock: output.BIOSNumLock,
            bios_serial_number: output.BIOSSerialNumber,
            boot_order: output.BootOrder.into_iter().map(|value| super::DeviceType::try_from(value).map_err(serde::de::Error::custom)).collect::<Result<_, _>>()?,
            boot_source_order: output.BootSourceOrder,
            caption: output.Caption,
            chassis_asset_tag: output.ChassisAssetTag,
            chassis_serial_number: output.ChassisSerialNumber,
            configuration_data_root: PathBuf::from(output.ConfigurationDataRoot),
            configuration_file: PathBuf::from(output.ConfigurationFile),
            configuration_id: output.ConfigurationID,
            console_mode: output.ConsoleMode.try_into().map_err(serde::de::Error::custom)?,
            creation_time: output.CreationTime.0.with_timezone(&chrono::Utc),
            debug_channel_id: output.DebugChannelId,
            debug_port: output.DebugPort.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            debug_port_enabled: output.DebugPortEnabled.map(TryInto::try_into).transpose().map_err(serde::de::Error::custom)?,
            description: output.Description,
            element_name: output.ElementName,
            enhanced_session_transport_type: output.EnhancedSessionTransportType.try_into().map_err(serde::de::Error::custom)?,
            guest_controlled_cache_types: output.GuestControlledCacheTypes,
            guest_state_data_root: PathBuf::from(output.GuestStateDataRoot),
            guest_state_file: PathBuf::from(output.GuestStateFile),
            high_mmio_gap_size: output.HighMmioGapSize.into(),
            incremental_backup_enabled: output.IncrementalBackupEnabled,
            instance_id: output.InstanceID.into(),
            is_automatic_snapshot: output.IsAutomaticSnapshot,
            is_saved: output.IsSaved,
            lock_on_disconnect: output.LockOnDisconnect,
            log_data_root: output.LogDataRoot.map(PathBuf::from),
            low_mmio_gap_size: output.LowMmioGapSize.try_into().map_err(serde::de::Error::custom)?,
            network_boot_preferred_protocol: output.NetworkBootPreferredProtocol.try_into().map_err(serde::de::Error::custom)?,
            notes: output.Notes,
            parent: output.Parent,
            parent_package: output.ParentPackage,
            pause_after_boot_failure: output.PauseAfterBootFailure,
            recovery_file: output.RecoveryFile.map(PathBuf::from),
            secure_boot_enabled: output.SecureBootEnabled,
            secure_boot_template_id: output.SecureBootTemplateId,
            snapshot_data_root: output.SnapshotDataRoot.map(PathBuf::from),
            suspend_data_root: PathBuf::from(output.SuspendDataRoot),
            swap_file_data_root: output.SwapFileDataRoot.map(PathBuf::from),
            user_snapshot_type: output.UserSnapshotType.try_into().map_err(serde::de::Error::custom)?,
            version: output.Version.try_into().map_err(serde::de::Error::custom)?,
            virtual_numa_enabled: output.VirtualNumaEnabled,
            virtual_system_identifier,
            virtual_system_sub_type: output.VirtualSystemSubType,
            virtual_system_type: output.VirtualSystemType,
        })
    }
}
