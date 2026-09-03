extern crate core as backup_core;
extern crate std as core;

use backup_core::model::{Host, ReferencePointId, VmId};
use backup_core::service::referencepointservice::{ConsistencyLevel, ReferencePointCreateRequest, ReferencePointService, ReferencePointSettingData, ReferencePointType};
fn reference_point_settings() -> ReferencePointSettingData {
    ReferencePointSettingData::new(ConsistencyLevel::CrashConsistent)
}

async fn create_request() -> ReferencePointCreateRequest {
    ReferencePointCreateRequest {
        affected_system: Host::new().get_virtual_machine(resolve_vm_id().await).await.expect("Hyper-V VM should be queryable"),
        reference_point_settings: Some(reference_point_settings()),
        reference_point_type: ReferencePointType::RctBased,
        resulting_reference_point: None,
    }
}

async fn resolve_vm_id() -> VmId {
    let vm_name = std::env::var("HYPER_V_VM_NAME").expect("HYPER_V_VM_NAME must be set");
    Host::new()
        .get_virtual_machines()
        .await
        .expect("Hyper-V VMs should be queryable")
        .into_iter()
        .find(|virtual_machine| virtual_machine.name() == Some(vm_name.as_str()))
        .map(|virtual_machine| virtual_machine.id())
        .unwrap_or_else(|| panic!("no Hyper-V VM found with ElementName {vm_name:?}"))
}

async fn create_reference_point(service: &ReferencePointService) -> ReferencePointId {
    service.create(&create_request().await).await.expect("reference point should be created")
}

#[cfg(windows)]
#[tokio::test]
#[ignore = "requires a configured Hyper-V VM and reference-point settings"]
async fn creates_reference_point_with_wmi() {
    let service = ReferencePointService::new().await.expect("Hyper-V reference-point service should be available");
    let reference_point = create_reference_point(&service).await;

    service.destroy(reference_point).await.expect("reference point should be destroyed");
}

#[cfg(windows)]
#[tokio::test]
#[ignore = "requires a configured Hyper-V VM and reference-point settings"]
async fn cleanup_destroys_reference_point_when_not_retained() {
    let service = ReferencePointService::new().await.expect("Hyper-V reference-point service should be available");
    let reference_point = create_reference_point(&service).await;

    service.cleanup(reference_point, false).await.expect("cleanup should destroy the reference point");
}

#[cfg(windows)]
#[tokio::test]
#[ignore = "requires a configured Hyper-V VM and reference-point settings"]
async fn cleanup_removes_associated_data_when_retained() {
    let service = ReferencePointService::new().await.expect("Hyper-V reference-point service should be available");
    let reference_point = create_reference_point(&service).await;

    service.cleanup(reference_point, true).await.expect("cleanup should remove associated data");
    service.destroy(reference_point).await.expect("reference point should be destroyed after retained cleanup");
}
