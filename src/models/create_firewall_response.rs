/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateFirewallResponse : Response to POST https://api.hetzner.cloud/v1/firewalls

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFirewallResponse {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::models::Action>>,
    #[serde(rename = "firewall", skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Box<crate::models::Firewall>>,
}

impl CreateFirewallResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/firewalls
    pub fn new() -> CreateFirewallResponse {
        CreateFirewallResponse {
            actions: None,
            firewall: None,
        }
    }
}
