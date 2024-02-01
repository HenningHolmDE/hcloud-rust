/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ApplyToResourcesRequest : Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplyToResourcesRequest {
    /// Resources the Firewall should be applied to
    #[serde(rename = "apply_to")]
    pub apply_to: Vec<crate::models::FirewallResource>,
}

impl ApplyToResourcesRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources
    pub fn new(apply_to: Vec<crate::models::FirewallResource>) -> ApplyToResourcesRequest {
        ApplyToResourcesRequest { apply_to }
    }
}
