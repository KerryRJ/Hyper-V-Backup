use super::{Path, VirtualSystemReferencePoint, VirtualSystemSettingData};

pub(crate) struct VirtualMachine {
    connection: wmi::WMIConnection,
    pub(crate) path: Path,
}

impl VirtualMachine {
    pub(crate) fn from_path(connection: wmi::WMIConnection, path: String) -> Self {
        Self { connection, path: path.into() }
    }

    pub(super) async fn get_snapshots(&self) -> Vec<VirtualSystemSettingData> {
        self.get_relationships::<VirtualSystemSettingData>("Msvm_SettingsDefineState")
            .await
            .into_iter()
            .filter(VirtualSystemSettingData::is_snapshot)
            .collect()
    }

    async fn get_relationships<T>(&self, association_class: &str) -> Vec<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let query = format!("ASSOCIATORS OF {{{}}} WHERE AssocClass = {association_class}", self.path.as_str());
        self.connection.async_raw_query::<T>(&query).await.unwrap_or_default()
    }

    pub(super) async fn get_reference_points(&self) -> Vec<VirtualSystemReferencePoint> {
        self.get_relationships::<VirtualSystemReferencePoint>("Msvm_ReferencePointOfVirtualSystem").await
    }
}