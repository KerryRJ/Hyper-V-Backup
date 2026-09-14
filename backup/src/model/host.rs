use crate::infra::{ComputerSystemOut, HYPER_V_NAMESPACE};
use crate::model::{Error, VirtualMachine, VirtualMachineId};

pub struct Host {
    connection: wmi::WMIConnection,
}

const QUERY: &str = "SELECT * FROM Msvm_ComputerSystem WHERE (Caption = 'Virtual Machine')";

impl Host {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            connection: wmi::WMIConnection::with_namespace_path(HYPER_V_NAMESPACE)?,
        })
    }

    pub async fn get_virtual_machine(&self, id: VirtualMachineId) -> Result<VirtualMachine, Error> {
        let q = format!("{QUERY} AND Name = '{id}'");
        let machine = self.connection.async_raw_query::<ComputerSystemOut>(&q).await?.into_iter().next().ok_or(Error::VirtualMachineNotFound(id))?;
        Ok(VirtualMachine::from(machine, self.connection.clone())?)
    }

    pub async fn get_virtual_machine_by_name(&self, name: &str) -> Result<VirtualMachine, Error> {
        let q = format!("{QUERY} AND ElementName = '{name}'");
        let machine = self.connection.async_raw_query::<ComputerSystemOut>(&q).await?.into_iter().next().ok_or_else(|| Error::InvalidBackupRequest("virtual machine not found"))?;
        Ok(VirtualMachine::from(machine, self.connection.clone())?)
    }

    pub async fn get_virtual_machines(&self) -> Result<Vec<VirtualMachine>, Error> {
        self.connection
            .async_raw_query::<ComputerSystemOut>(QUERY)
            .await?
            .into_iter()
            .map(|machine| VirtualMachine::from(machine, self.connection.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_not_found_for_unknown_vm() {
        let missing_id = VirtualMachineId::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        let result = Host::new().unwrap().get_virtual_machine(missing_id).await;
        assert!(matches!(
            result,
            Err(Error::VirtualMachineNotFound(id)) if id == missing_id
        ));
    }

    #[tokio::test]
    #[ignore = "requires a running Hyper-V VM"]
    async fn returns_virtual_machine_by_id() {
        let vm_id = VirtualMachineId::parse_str("B12E125D-5EDF-40DA-94A0-89BA9836221D").unwrap();
        let vm = Host::new().unwrap().get_virtual_machine(vm_id).await.expect("expected Hyper-V VM to be found");
        assert_eq!(vm.name, vm_id);
        assert_eq!(vm.element_name, "vrt001");
    }

    #[tokio::test]
    #[ignore = "requires three running Hyper-V VMs"]
    async fn returns_three_virtual_machines() {
        let vms = Host::new().unwrap().get_virtual_machines().await.expect("expected Hyper-V VMs to be returned");
        assert_eq!(vms.len(), 3);
    }
}
