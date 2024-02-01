/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceVolumeRequest : Request for PUT https://api.hetzner.cloud/v1/volumes/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceVolumeRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New Volume name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceVolumeRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for PUT https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new() -> ReplaceVolumeRequest {
        ReplaceVolumeRequest {
            labels: None,
            name: None,
        }
    }
}
