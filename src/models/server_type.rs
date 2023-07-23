/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerType {
    #[serde(rename = "architecture")]
    pub architecture: crate::models::Architecture,
    /// Number of cpu cores a Server of this type will have
    #[serde(rename = "cores")]
    pub cores: i32,
    /// Type of cpu
    #[serde(rename = "cpu_type")]
    pub cpu_type: CpuType,
    /// This field is deprecated. Use the deprecation object instead
    #[serde(rename = "deprecated", deserialize_with = "Option::deserialize")]
    pub deprecated: Option<bool>,
    #[serde(
        rename = "deprecation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deprecation: Option<Option<Box<crate::models::DeprecationInfo>>>,
    /// Description of the Server type
    #[serde(rename = "description")]
    pub description: String,
    /// Disk size a Server of this type will have in GB
    #[serde(rename = "disk")]
    pub disk: f32,
    /// ID of the Server type
    #[serde(rename = "id")]
    pub id: i64,
    /// Free traffic per month in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
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
    pub fn new(
        architecture: crate::models::Architecture,
        cores: i32,
        cpu_type: CpuType,
        deprecated: Option<bool>,
        description: String,
        disk: f32,
        id: i64,
        included_traffic: i64,
        memory: f32,
        name: String,
        prices: Vec<crate::models::PricePerTime>,
        storage_type: StorageType,
    ) -> ServerType {
        ServerType {
            architecture,
            cores,
            cpu_type,
            deprecated,
            deprecation: None,
            description,
            disk,
            id,
            included_traffic,
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
