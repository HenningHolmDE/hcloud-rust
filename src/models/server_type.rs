/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServerType : Type of Server - determines how much ram, disk and cpu a Server has

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerType {
    /// Number of cpu cores a Server of this type will have
    #[serde(rename = "cores")]
    pub cores: i32,
    /// Type of cpu
    #[serde(rename = "cpu_type")]
    pub cpu_type: CpuType,
    /// True if Server type is deprecated
    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,
    /// Description of the Server type
    #[serde(rename = "description")]
    pub description: String,
    /// Disk size a Server of this type will have in GB
    #[serde(rename = "disk")]
    pub disk: f32,
    /// ID of the Server type
    #[serde(rename = "id")]
    pub id: i32,
    /// Memory a Server of this type will have in GB
    #[serde(rename = "memory")]
    pub memory: f32,
    /// Unique identifier of the Server type
    #[serde(rename = "name")]
    pub name: String,
    /// Prices in different Locations
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTime>,
    /// Type of Server boot drive. Local has higher speed. Network has better availability.
    #[serde(rename = "storage_type")]
    pub storage_type: StorageType,
}

impl ServerType {
    #![allow(clippy::too_many_arguments)]
    /// Type of Server - determines how much ram, disk and cpu a Server has
    pub fn new(
        cores: i32,
        cpu_type: CpuType,
        deprecated: Option<bool>,
        description: String,
        disk: f32,
        id: i32,
        memory: f32,
        name: String,
        prices: Vec<crate::models::PricePerTime>,
        storage_type: StorageType,
    ) -> ServerType {
        ServerType {
            cores,
            cpu_type,
            deprecated,
            description,
            disk,
            id,
            memory,
            name,
            prices,
            storage_type,
        }
    }
}

/// Type of cpu
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CpuType {
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "shared")]
    Shared,
}

impl Default for CpuType {
    fn default() -> CpuType {
        Self::Dedicated
    }
}
/// Type of Server boot drive. Local has higher speed. Network has better availability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "network")]
    Network,
}

impl Default for StorageType {
    fn default() -> StorageType {
        Self::Local
    }
}
