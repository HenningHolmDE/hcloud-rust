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

/// ReplaceFirewallRequest : Request for PUT https://api.hetzner.cloud/v1/firewalls/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceFirewallRequest {
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the [Firewall](#firewalls).  Limited to a maximum of 128 characters.  Must be unique per Project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceFirewallRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/firewalls/{id}
    pub fn new() -> ReplaceFirewallRequest {
        ReplaceFirewallRequest {
            labels: None,
            name: None,
        }
    }
}
