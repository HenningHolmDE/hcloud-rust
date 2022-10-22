/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeRequest : Request for POST https://api.hetzner.cloud/v1/volumes

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    /// Auto-mount Volume after attach. `server` must be provided.
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
    /// Format Volume after creation. One of: `xfs`, `ext4`
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Location to create the Volume in (can be omitted if Server is specified)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Name of the volume
    #[serde(rename = "name")]
    pub name: String,
    /// Server to which to attach the Volume once it's created (Volume will be created in the same Location as the server)
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<i32>,
    /// Size of the Volume in GB
    #[serde(rename = "size")]
    pub size: i32,
}

impl CreateVolumeRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/volumes
    pub fn new(name: String, size: i32) -> CreateVolumeRequest {
        CreateVolumeRequest {
            automount: None,
            format: None,
            labels: None,
            location: None,
            name,
            server: None,
            size,
        }
    }
}
