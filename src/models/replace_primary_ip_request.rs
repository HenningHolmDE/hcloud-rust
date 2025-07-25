/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ReplacePrimaryIpRequest : Request for PUT https://api.hetzner.cloud/v1/primary_ips/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacePrimaryIpRequest {
    /// Auto deletion state.  If enabled the [Primary IP](#primary-ips) will be deleted once the assigned resource gets deleted.
    #[serde(rename = "auto_delete", skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".  | User-defined labels (`key/value` pairs) for the Resource.  Note that the set of [Labels](#labels) provided in the request will overwrite the existing one.  For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplacePrimaryIpRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/primary_ips/{id}
    pub fn new() -> ReplacePrimaryIpRequest {
        ReplacePrimaryIpRequest {
            auto_delete: None,
            labels: None,
            name: None,
        }
    }
}
