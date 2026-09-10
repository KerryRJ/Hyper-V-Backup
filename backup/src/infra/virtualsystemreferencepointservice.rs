
use std::path::PathBuf;
use super::*;

pub(super) struct VirtualSystemReferencePointService {
    connection: wmi::WMIConnection,
    path: String,
}

impl VirtualSystemReferencePointService {
    pub(super) fn new(connection: wmi::WMIConnection) -> wmi::WMIResult<Self> {
        let service = connection
            .raw_query::<VirtualSystemReferencePointServiceOut>("SELECT __Path FROM Msvm_VirtualSystemReferencePointService")?
            .into_iter()
            .next()
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("Virtual system reference point service not found".into()))?;
        Ok(Self { connection, path: service.path })
    }

    pub(super) async fn create(&self, affected_system: &VirtualMachine, reference_point_settings: super::VirtualSystemReferencePointSettingDataIn, reference_point_type: ReferencePointType, resulting_reference_point_type: Option<&ReferencePointType>) -> wmi::WMIResult<VirtualSystemReferencePoint> {
        unimplemented!()
    }

    pub(super) async fn export(&self, reference_point: &VirtualSystemReferencePoint) -> wmi::WMIResult<()> {
        unimplemented!()
    }

    pub(super) async fn destroy(&self, affected_reference_point: VirtualSystemReferencePoint) -> wmi::WMIResult<()> {
        #[derive(Debug, PartialEq, Eq)]
        enum ReturnValue {
            Completed,
            MethodParametersCheckedAndJobStarted,
            Failed,
            AccessDenied,
            NotSupported,
            StatusUnknown,
            Timeout,
            InvalidParameter,
            SystemInUse,
            InvalidState,
            IncorrectDataType,
            SystemNotAvailable,
            OutOfMemory,
        }

        impl TryFrom<u32> for ReturnValue {
            type Error = u32;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::Completed),
                    4096 => Ok(Self::MethodParametersCheckedAndJobStarted),
                    32768 => Ok(Self::Failed),
                    32769 => Ok(Self::AccessDenied),
                    32770 => Ok(Self::NotSupported),
                    32771 => Ok(Self::StatusUnknown),
                    32772 => Ok(Self::Timeout),
                    32773 => Ok(Self::InvalidParameter),
                    32774 => Ok(Self::SystemInUse),
                    32775 => Ok(Self::InvalidState),
                    32776 => Ok(Self::IncorrectDataType),
                    32777 => Ok(Self::SystemNotAvailable),
                    32778 => Ok(Self::OutOfMemory),
                    value => Err(value),
                }
            }
        }

        impl std::fmt::Display for ReturnValue {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let description = match self {
                    Self::Completed => "Completed with no error",
                    Self::MethodParametersCheckedAndJobStarted => "Method parameters checked - job started",
                    Self::Failed => "Failed",
                    Self::AccessDenied => "Access denied",
                    Self::NotSupported => "Not supported",
                    Self::StatusUnknown => "Status is unknown",
                    Self::Timeout => "Timeout",
                    Self::InvalidParameter => "Invalid parameter",
                    Self::SystemInUse => "System is in use",
                    Self::InvalidState => "Invalid state for this operation",
                    Self::IncorrectDataType => "Incorrect data type",
                    Self::SystemNotAvailable => "System is not available",
                    Self::OutOfMemory => "Out of memory",
                };
                formatter.write_str(description)
            }
        }

        let destroy_reference_point_method_class = self
            .connection
            .get_object("Msvm_VirtualSystemReferencePointService")?
            .get_method("DestroyReferencePoint ")?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint method signature not found".into()))?;

        let mut job_events = self
            .connection
            .async_raw_notification::<ConcreteJobModificationEvent>("SELECT TargetInstance FROM __InstanceModificationEvent WITHIN 1 WHERE TargetInstance ISA 'Msvm_ConcreteJob'")?;

        let input = destroy_reference_point_method_class.spawn_instance()?;
        input
            .put_property("AffectedReferencePoint", affected_reference_point.path.as_str().to_owned())
            .map_err(|error| wmi::WMIError::ConvertVariantError(format!("Failed to set AffectedReferencePoint: {error}").into()))?;

        let result = self
            .connection
            .exec_method(&self.path, "DestroyReferencePoint", Some(&input))?
            .ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no output".into()))?
            .into_desr::<MethodResult>()?;

        let return_value = ReturnValue::try_from(result.return_value).map_err(|value| {
            wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint returned unknown status {value}").into())
        })?;
        log::debug!("DestroyReferencePoint returned {return_value} ({})", result.return_value);

        match return_value {
            ReturnValue::Completed => return Ok(()),
            ReturnValue::MethodParametersCheckedAndJobStarted => {
                let path = result.job.ok_or_else(|| wmi::WMIError::ConvertVariantError("DestroyReferencePoint returned no job".into()))?;
                Job::wait(&self.connection, path, &mut job_events).await?;
            }
            return_value => {
                return Err(wmi::WMIError::ConvertVariantError(format!("DestroyReferencePoint failed: {return_value} ({})", result.return_value).into()));
            }
        }

        Ok(())
    }

    pub(super) async fn remove_associated_data(&self, affected_reference_point: VirtualSystemReferencePoint) -> wmi::WMIResult<()> {
        unimplemented!()
    }

    pub(super) async fn import(&self, affected_system: &VirtualMachine, config_file_path: PathBuf, runtime_state_file_path: PathBuf) -> wmi::WMIResult<VirtualMachine> {
        unimplemented!()
    }
}
