/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ReplaceFloatingIpRequest : Request for PUT https://api.hetzner.cloud/v1/floating_ips/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceFloatingIpRequest {
    /// Description of the Resource.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceFloatingIpRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/floating_ips/{id}
    pub fn new() -> ReplaceFloatingIpRequest {
        ReplaceFloatingIpRequest {
            description: None,
            labels: None,
            name: None,
        }
    }
}
