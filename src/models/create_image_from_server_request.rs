/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateImageFromServerRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/create_image

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateImageFromServerRequest {
    /// Description of the Image, will be auto-generated if not set
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Type of Image to create (default: `snapshot`)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl CreateImageFromServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/create_image
    pub fn new() -> CreateImageFromServerRequest {
        CreateImageFromServerRequest {
            description: None,
            labels: None,
            _type: None,
        }
    }
}

/// Type of Image to create (default: `snapshot`)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "snapshot")]
    Snapshot,
}

impl Default for Type {
    fn default() -> Type {
        Self::Backup
    }
}
