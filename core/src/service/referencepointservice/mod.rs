use crate::model::Error;
use crate::model::ReferencePoint;
use crate::wmi::msvm::virtualsystemreferencepointservice::VirtualSystemReferencePointService as WmiReferencePointService;

pub struct ReferencePointService {
    wmi: WmiReferencePointService,
}

impl ReferencePointService {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            wmi: WmiReferencePointService::new().await?,
        })
    }

    pub async fn create_reference_point(&self, affected_system: &str, reference_point_settings: &str, reference_point_type: u16, resulting_reference_point: &str) -> Result<ReferencePoint, Error> {
        let result = self
            .wmi
            .create_reference_point(
                affected_system,
                reference_point_settings,
                reference_point_type,
                resulting_reference_point,
            )
            .await?;
        Ok(WmiReferencePointService::reference_point(result)?)
    }

    pub async fn destroy_reference_point(&self, reference_point: &ReferencePoint) -> Result<(), Error> {
        self.wmi
            .destroy_reference_point(&reference_point.id().to_string())
            .await?;
        Ok(())
    }

    pub async fn remove_associated_data(&self, reference_point: &ReferencePoint) -> Result<(), Error> {
        self.wmi
            .remove_associated_data(&reference_point.id().to_string())
            .await?;
        Ok(())
    }

        pub async fn cleanup_reference_point(
            &self,
            reference_point: &ReferencePoint,
            retain_for_incremental: bool,
        ) -> Result<(), Error> {
            if retain_for_incremental {
                self.remove_associated_data(reference_point).await
            } else {
                self.destroy_reference_point(reference_point).await
            }
        }
}