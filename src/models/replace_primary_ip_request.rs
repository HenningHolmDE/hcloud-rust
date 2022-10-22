/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplacePrimaryIpRequest : Request for PUT https://api.hetzner.cloud/v1/primary_ips/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplacePrimaryIpRequest {
    /// Delete this Primary IP when the resource it is assigned to is deleted
    #[serde(rename = "auto_delete", skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New unique name to set
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplacePrimaryIpRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for PUT https://api.hetzner.cloud/v1/primary_ips/{id}
    pub fn new() -> ReplacePrimaryIpRequest {
        ReplacePrimaryIpRequest {
            auto_delete: None,
            labels: None,
            name: None,
        }
    }
}
