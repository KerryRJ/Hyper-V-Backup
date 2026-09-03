mod referencepointcreaterequest;
mod referencepointsettingdata;

use crate::model::Error;
use crate::model::ReferencePoint;
use crate::wmi::msvm::virtualsystemreferencepointservice::VirtualSystemReferencePointService as WmiReferencePointService;

pub struct ReferencePointService {
    wmi: WmiReferencePointService,
}

pub use crate::model::ConsistencyLevel;
pub use crate::model::ReferencePointType;
pub use referencepointcreaterequest::ReferencePointCreateRequest;
pub use referencepointsettingdata::ReferencePointSettingData;

impl ReferencePointService {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self { wmi: WmiReferencePointService::new().await? })
    }

    pub async fn create(&self, request: &ReferencePointCreateRequest) -> Result<ReferencePoint, Error> {
        let result = self
            .wmi
            .create_reference_point(&request.affected_system, request.reference_point_settings.as_ref(), request.reference_point_type as u16, request.resulting_reference_point)
            .await?;
        Ok(self.wmi.reference_point(result).await?)
    }

    pub async fn destroy(&self, reference_point: &ReferencePoint) -> Result<(), Error> {
        self.wmi.destroy_reference_point(reference_point.path().as_str()).await?;
        Ok(())
    }

    pub async fn remove_associated_data(&self, reference_point: &ReferencePoint) -> Result<(), Error> {
        self.wmi.remove_associated_data(reference_point.path().as_str()).await?;
        Ok(())
    }

    pub async fn cleanup(&self, reference_point: &ReferencePoint, retain_for_incremental: bool) -> Result<(), Error> {
        if retain_for_incremental {
            self.remove_associated_data(reference_point).await
        } else {
            self.destroy(reference_point).await
        }
    }
}
