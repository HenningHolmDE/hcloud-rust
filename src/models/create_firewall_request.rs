/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateFirewallRequest : Request for POST https://api.hetzner.cloud/v1/firewalls
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFirewallRequest {
    /// Resources the Firewall should be applied to after creation
    #[serde(rename = "apply_to", skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<Vec<models::FirewallResource>>,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the Firewall.  Limited to a maximum of 128 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of rules.  Limited to a maximum of 50 rules per Firewall.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<models::Rule>>,
}

impl CreateFirewallRequest {
    /// Request for POST https://api.hetzner.cloud/v1/firewalls
    pub fn new(name: String) -> CreateFirewallRequest {
        CreateFirewallRequest {
            apply_to: None,
            labels: None,
            name,
            rules: None,
        }
    }
}
