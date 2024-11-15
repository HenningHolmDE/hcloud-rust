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

/// ReplaceSshKeyRequest : Request for PUT https://api.hetzner.cloud/v1/ssh_keys/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSshKeyRequest {
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// New name Name to set
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceSshKeyRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/ssh_keys/{id}
    pub fn new() -> ReplaceSshKeyRequest {
        ReplaceSshKeyRequest {
            labels: None,
            name: None,
        }
    }
}
