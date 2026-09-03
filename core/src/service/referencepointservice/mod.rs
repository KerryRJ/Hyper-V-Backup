mod consistencylevel;
mod referencepointcreaterequest;
mod referencepointsettingdata;
mod referencepointtype;

use crate::model::Error;
use crate::model::ReferencePointId;
use crate::wmi::msvm::virtualsystemreferencepointservice::VirtualSystemReferencePointService as WmiReferencePointService;

pub struct ReferencePointService {
    wmi: WmiReferencePointService,
}

pub use consistencylevel::ConsistencyLevel;
pub use referencepointcreaterequest::ReferencePointCreateRequest;
pub use referencepointsettingdata::ReferencePointSettingData;
pub use referencepointtype::ReferencePointType;

impl ReferencePointService {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self { wmi: WmiReferencePointService::new().await? })
    }

    pub async fn create(&self, request: &ReferencePointCreateRequest) -> Result<ReferencePointId, Error> {
        let result = self
            .wmi
            .create_reference_point(&request.affected_system, request.reference_point_settings.as_ref(), request.reference_point_type as u16, request.resulting_reference_point)
            .await?;
        Ok(WmiReferencePointService::reference_point(result)?)
    }

    pub async fn destroy(&self, reference_point_id: ReferencePointId) -> Result<(), Error> {
        self.wmi.destroy_reference_point(&reference_point_id.to_string()).await?;
        Ok(())
    }

    pub async fn remove_associated_data(&self, reference_point_id: ReferencePointId) -> Result<(), Error> {
        self.wmi.remove_associated_data(&reference_point_id.to_string()).await?;
        Ok(())
    }

    pub async fn cleanup(&self, reference_point_id: ReferencePointId, retain_for_incremental: bool) -> Result<(), Error> {
        if retain_for_incremental {
            self.remove_associated_data(reference_point_id).await
        } else {
            self.destroy(reference_point_id).await
        }
    }
}
