/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceFloatingIpRequest : Request for PUT https://api.hetzner.cloud/v1/floating_ips/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceFloatingIpRequest {
    /// New Description to set
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New unique name to set
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceFloatingIpRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for PUT https://api.hetzner.cloud/v1/floating_ips/{id}
    pub fn new() -> ReplaceFloatingIpRequest {
        ReplaceFloatingIpRequest {
            description: None,
            labels: None,
            name: None,
        }
    }
}
