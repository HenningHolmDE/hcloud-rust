/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    /// ID of Server the Image is bound to. Only set for Images of type `backup`.
    #[serde(rename = "bound_to", deserialize_with = "Option::deserialize")]
    pub bound_to: Option<i32>,
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "created_from", deserialize_with = "Option::deserialize")]
    pub created_from: Option<Box<crate::models::CreatedFrom>>,
    /// Point in time where the Image was deleted (in ISO-8601 format)
    #[serde(rename = "deleted", deserialize_with = "Option::deserialize")]
    pub deleted: Option<String>,
    /// Point in time when the Image is considered to be deprecated (in ISO-8601 format)
    #[serde(rename = "deprecated", deserialize_with = "Option::deserialize")]
    pub deprecated: Option<String>,
    /// Description of the Image
    #[serde(rename = "description")]
    pub description: String,
    /// Size of the disk contained in the Image in GB
    #[serde(rename = "disk_size")]
    pub disk_size: f32,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
    #[serde(rename = "image_size", deserialize_with = "Option::deserialize")]
    pub image_size: Option<f32>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Unique identifier of the Image. This value is only set for system Images.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Flavor of operating system contained in the Image
    #[serde(rename = "os_flavor")]
    pub os_flavor: OsFlavor,
    /// Operating system version
    #[serde(rename = "os_version", deserialize_with = "Option::deserialize")]
    pub os_version: Option<String>,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// Indicates that rapid deploy of the Image is available
    #[serde(rename = "rapid_deploy", skip_serializing_if = "Option::is_none")]
    pub rapid_deploy: Option<bool>,
    /// Whether the Image can be used or if it's still being created or unavailable
    #[serde(rename = "status")]
    pub status: Status,
    /// Type of the Image
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl Image {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        bound_to: Option<i32>,
        created: String,
        created_from: Option<crate::models::CreatedFrom>,
        deleted: Option<String>,
        deprecated: Option<String>,
        description: String,
        disk_size: f32,
        id: i32,
        image_size: Option<f32>,
        labels: ::std::collections::HashMap<String, String>,
        name: Option<String>,
        os_flavor: OsFlavor,
        os_version: Option<String>,
        protection: crate::models::Protection,
        status: Status,
        r#type: Type,
    ) -> Image {
        Image {
            bound_to,
            created,
            created_from: created_from.map(Box::new),
            deleted,
            deprecated,
            description,
            disk_size,
            id,
            image_size,
            labels,
            name,
            os_flavor,
            os_version,
            protection: Box::new(protection),
            rapid_deploy: None,
            status,
            r#type,
        }
    }
}

/// Flavor of operating system contained in the Image
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OsFlavor {
    #[serde(rename = "centos")]
    Centos,
    #[serde(rename = "debian")]
    Debian,
    #[serde(rename = "fedora")]
    Fedora,
    #[serde(rename = "rocky")]
    Rocky,
    #[serde(rename = "ubuntu")]
    Ubuntu,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for OsFlavor {
    fn default() -> OsFlavor {
        Self::Centos
    }
}
/// Whether the Image can be used or if it's still being created or unavailable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl Default for Status {
    fn default() -> Status {
        Self::Available
    }
}
/// Type of the Image
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "temporary")]
    Temporary,
}

impl Default for Type {
    fn default() -> Type {
        Self::App
    }
}
