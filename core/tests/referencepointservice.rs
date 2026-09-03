extern crate core as backup_core;
extern crate std as core;

use backup_core::model::{ReferencePointId, VmId};
use backup_core::service::referencepointservice::{ReferencePointCreateRequest, ReferencePointService, ReferencePointSettingsData, ReferencePointType};
use serde::Deserialize;
use wmi::WMIConnection;

const HYPER_V_NAMESPACE: &str = r"ROOT\virtualization\v2";

fn reference_point_settings() -> ReferencePointSettingsData {
    ReferencePointSettingsData::new()
}

#[derive(Deserialize)]
struct VirtualMachineId {
    #[serde(rename = "Name")]
    id: VmId,
}

async fn create_request() -> ReferencePointCreateRequest {
    ReferencePointCreateRequest {
        affected_system: resolve_vm_id().await,
        reference_point_settings: Some(reference_point_settings()),
        reference_point_type: ReferencePointType::RctBased,
        resulting_reference_point: None,
    }
}

async fn resolve_vm_id() -> VmId {
    let vm_name = std::env::var("HYPER_V_VM_NAME").expect("HYPER_V_VM_NAME must be set");
    let escaped_vm_name = vm_name.replace('\'', "''");
    let query = format!("SELECT Name FROM Msvm_ComputerSystem WHERE ElementName = '{escaped_vm_name}'");
    let connection = WMIConnection::with_namespace_path(HYPER_V_NAMESPACE).expect("Hyper-V WMI connection should be available");
    let mut virtual_machines = connection.async_raw_query::<VirtualMachineId>(&query).await.expect("Hyper-V VM should be queryable");

    match virtual_machines.len() {
        1 => virtual_machines.remove(0).id,
        0 => panic!("no Hyper-V VM found with ElementName {vm_name:?}"),
        count => panic!("expected one Hyper-V VM named {vm_name:?}, found {count}"),
    }
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
