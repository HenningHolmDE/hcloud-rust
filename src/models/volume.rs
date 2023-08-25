/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Volume : A Volume is a highly-available, scalable, and SSD-based block storage for Servers. Pricing for Volumes depends on the Volume size and Location, not the actual used storage. Please see [Hetzner Wiki](https://wiki.hetzner.de/index.php/CloudServer/en#Volumes) for more details about Volumes.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Volume {
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Filesystem of the Volume if formatted on creation, null if not formatted on creation
    #[serde(rename = "format", deserialize_with = "Option::deserialize")]
    pub format: Option<String>,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i64,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Device path on the file system for the Volume
    #[serde(rename = "linux_device")]
    pub linux_device: String,
    #[serde(rename = "location")]
    pub location: Box<crate::models::Location>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// ID of the Server the Volume is attached to, null if it is not attached at all
    #[serde(rename = "server", deserialize_with = "Option::deserialize")]
    pub server: Option<i64>,
    /// Size in GB of the Volume
    #[serde(rename = "size")]
    pub size: f32,
    /// Current status of the Volume
    #[serde(rename = "status")]
    pub status: Status,
}

impl Volume {
    #![allow(clippy::too_many_arguments)]
    /// A Volume is a highly-available, scalable, and SSD-based block storage for Servers. Pricing for Volumes depends on the Volume size and Location, not the actual used storage. Please see [Hetzner Wiki](https://wiki.hetzner.de/index.php/CloudServer/en#Volumes) for more details about Volumes.
    pub fn new(
        created: String,
        format: Option<String>,
        id: i64,
        labels: ::std::collections::HashMap<String, String>,
        linux_device: String,
        location: crate::models::Location,
        name: String,
        protection: crate::models::Protection,
        server: Option<i64>,
        size: f32,
        status: Status,
    ) -> Volume {
        Volume {
            created,
            format,
            id,
            labels,
            linux_device,
            location: Box::new(location),
            name,
            protection: Box::new(protection),
            server,
            size,
            status,
        }
    }
}

/// Current status of the Volume
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "creating")]
    Creating,
}

impl Default for Status {
    fn default() -> Status {
        Self::Available
    }
}
