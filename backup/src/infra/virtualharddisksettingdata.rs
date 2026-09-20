use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::infra::InstanceId;
use super::*;

#[derive(Clone)]
pub(crate) struct VirtualHardDiskSettingData {
    block_size: u32,
    caption: String,
    data_alignment: Option<u64>,
    description: String,
    element_name: String,
    format: Format,
    instance_id: InstanceId,
    is_pmem_compatible: bool,
    logical_sector_size: u32,
    max_internal_size: u64,
    parent_identifier: Option<ParentId>,
    parent_path: Path,
    parent_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) path: Path,
    physical_sector_size: u32,
    pmem_address_abstraction_type: PmemAddressAbstractionType,
    _type: Type,
    virtual_disk_id: VirtualDiskId,
}

impl fmt::Debug for VirtualHardDiskSettingData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VirtualHardDiskSettingData")
            .field("block_size", &self.block_size)
            .field("caption", &self.caption)
            .field("data_alignment", &self.data_alignment)
            .field("description", &self.description)
            .field("element_name", &self.element_name)
            .field("format", &self.format)
            .field("instance_id", &self.instance_id)
            .field("is_pmem_compatible", &self.is_pmem_compatible)
            .field("logical_sector_size", &self.logical_sector_size)
            .field("max_internal_size", &self.max_internal_size)
            .field("parent_identifier", &self.parent_identifier)
            .field("parent_path", &self.parent_path)
            .field("parent_timestamp", &self.parent_timestamp)
            .field("path", &self.path)
            .field("physical_sector_size", &self.physical_sector_size)
            .field("pmem_address_abstraction_type", &self.pmem_address_abstraction_type)
            .field("_type", &self._type)
            .field("virtual_disk_id", &self.virtual_disk_id)
            .finish()
    }
}

impl VirtualHardDiskSettingData {
    pub(crate) fn max_internal_size(&self) -> u64 {
        self.max_internal_size
    }

    pub(super) fn instance_id(&self) -> &str {
        self.instance_id.as_ref()
    }

    pub(crate) fn from_embedded_xml(xml: &str) -> Result<Self, String> {
        let instance: EmbeddedInstance = quick_xml::de::from_str(xml).map_err(|error| error.to_string())?;
        let properties = instance
            .properties
            .into_iter()
            .filter_map(|property| property.value.map(|value| (property.name, value)))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            block_size: parse_required(&properties, "BlockSize")?,
            caption: required(&properties, "Caption")?.to_owned(),
            data_alignment: parse_optional(&properties, "DataAlignment")?,
            description: required(&properties, "Description")?.to_owned(),
            element_name: required(&properties, "ElementName")?.to_owned(),
            format: parse_required::<u16>(&properties, "Format")?.try_into().map_err(|error| format!("Format is invalid: {error}"))?,
            instance_id: required(&properties, "InstanceID")?.to_owned().into(),
            is_pmem_compatible: parse_bool_required(&properties, "IsPmemCompatible")?,
            logical_sector_size: parse_required(&properties, "LogicalSectorSize")?,
            max_internal_size: parse_required(&properties, "MaxInternalSize")?,
            parent_identifier: optional(&properties, "ParentIdentifier")
                .filter(|value| !value.is_empty())
                .map(ParentId::parse_str)
                .transpose()
                .map_err(|error| error.to_string())?,
            parent_path: required(&properties, "ParentPath")?.to_owned().into(),
            parent_timestamp: None,
            path: required(&properties, "Path")?.to_owned().into(),
            physical_sector_size: parse_required(&properties, "PhysicalSectorSize")?,
            pmem_address_abstraction_type: parse_required::<u16>(&properties, "PmemAddressAbstractionType")?
                .try_into()
                .map_err(|error| format!("PmemAddressAbstractionType is invalid: {error}"))?,
            _type: parse_required::<u16>(&properties, "Type")?.try_into().map_err(|error| format!("Type is invalid: {error}"))?,
            virtual_disk_id: required(&properties, "VirtualDiskId")?.to_owned().into(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct EmbeddedInstance {
    #[serde(rename = "PROPERTY", default)]
    properties: Vec<EmbeddedProperty>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedProperty {
    #[serde(rename = "@NAME")]
    name: String,
    #[serde(rename = "VALUE")]
    value: Option<String>,
}

fn required<'a>(properties: &'a HashMap<String, String>, name: &str) -> Result<&'a str, String> {
    optional(properties, name).ok_or_else(|| format!("missing property `{name}`"))
}

fn optional<'a>(properties: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    properties.get(name).map(String::as_str)
}

fn parse_required<T>(properties: &HashMap<String, String>, name: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
{
    required(properties, name)?.parse().map_err(|error| format!("{name} is invalid: {error}"))
}

fn parse_optional<T>(properties: &HashMap<String, String>, name: &str) -> Result<Option<T>, String>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
{
    optional(properties, name).filter(|value| !value.is_empty()).map(|value| value.parse().map_err(|error| format!("{name} is invalid: {error}"))).transpose()
}

fn parse_bool_required(properties: &HashMap<String, String>, name: &str) -> Result<bool, String> {
    match required(properties, name)? {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        value => Err(format!("{name} is invalid: {value}")),
    }
}

impl<'de> Deserialize<'de> for VirtualHardDiskSettingData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let output = VirtualHardDiskSettingDataOut::deserialize(deserializer)?;

        Ok(Self {
            block_size: output.BlockSize,
            caption: output.Caption,
            data_alignment: output.DataAlignment,
            description: output.Description,
            element_name: output.ElementName,
            format: output.Format.try_into().map_err(serde::de::Error::custom)?,
            instance_id: output.InstanceID.into(),
            is_pmem_compatible: output.IsPmemCompatible,
            logical_sector_size: output.LogicalSectorSize,
            max_internal_size: output.MaxInternalSize,
            parent_identifier: output
                .ParentIdentifier
                .filter(|value| !value.is_empty())
                .map(|value| ParentId::parse_str(&value))
                .transpose()
                .map_err(serde::de::Error::custom)?,
            parent_path: output.ParentPath.into(),
            parent_timestamp: output.ParentTimestamp.map(|value| value.0.with_timezone(&chrono::Utc)),
            path: output.Path.into(),
            physical_sector_size: output.PhysicalSectorSize,
            pmem_address_abstraction_type: output.PmemAddressAbstractionType.try_into().map_err(serde::de::Error::custom)?,
            _type: output.Type.try_into().map_err(serde::de::Error::custom)?,
            virtual_disk_id: output.VirtualDiskId.into(),
        })
    }
}
