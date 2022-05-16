/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceImageRequest : Request for PUT https://api.hetzner.cloud/v1/images/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceImageRequest {
    /// New description of Image
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Destination Image type to convert to
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl ReplaceImageRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/images/{id}
    pub fn new() -> ReplaceImageRequest {
        ReplaceImageRequest {
            description: None,
            labels: None,
            _type: None,
        }
    }
}

/// Destination Image type to convert to
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "snapshot")]
    Snapshot,
}

impl Default for Type {
    fn default() -> Type {
        Self::Snapshot
    }
}
