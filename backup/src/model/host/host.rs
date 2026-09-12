use crate::model::Error;
use crate::model::{VirtualMachine};
use crate::infra::VirtualMachineId;
use crate::infra::ComputerSystemOut;
use crate::infra::HYPER_V_NAMESPACE;

pub struct Host;

const QUERY: &str = "SELECT * FROM Msvm_ComputerSystem WHERE (Caption = 'Virtual Machine')";

impl Host {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_virtual_machine(&self, id: VirtualMachineId) -> Result<VirtualMachine, Error> {
        let q = format!("{QUERY} AND Name = '{id}'");
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)?;
        let machine = connection.async_raw_query::<ComputerSystemOut>(&q).await?.into_iter().next().ok_or(Error::VirtualMachineNotFound(id))?;
        Ok(machine.try_into()?)
    }

    pub async fn get_virtual_machines(&self) -> Result<Vec<VirtualMachine>, Error> {
        let connection = wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)?;
        connection
            .async_raw_query::<ComputerSystemOut>(QUERY)
            .await?
            .into_iter()
            .map(VirtualMachine::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::VirtualMachineId;

    #[tokio::test]
    async fn returns_not_found_for_unknown_vm() {
        let missing_id = VirtualMachineId::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        let result = Host::new().get_virtual_machine(missing_id).await;
        assert!(matches!(
            result,
            Err(Error::VirtualMachineNotFound(id)) if id == missing_id
        ));
    }

    #[tokio::test]
    #[ignore = "requires a running Hyper-V VM"]
    async fn returns_virtual_machine_by_id() {
        let vm_id = VirtualMachineId::parse_str("B12E125D-5EDF-40DA-94A0-89BA9836221D").unwrap();
        let vm = Host::new().get_virtual_machine(vm_id).await.expect("expected Hyper-V VM to be found");
        assert_eq!(vm.id(), vm_id);
        assert_eq!(vm.name(), Some("vrt001"));
    }

    #[tokio::test]
    #[ignore = "requires three running Hyper-V VMs"]
    async fn returns_three_virtual_machines() {
        let vms = Host::new().get_virtual_machines().await.expect("expected Hyper-V VMs to be returned");
        assert_eq!(vms.len(), 3);
    }
}
