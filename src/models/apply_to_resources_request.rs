/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApplyToResourcesRequest : Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyToResourcesRequest {
    /// Resources the Firewall should be applied to
    #[serde(rename = "apply_to")]
    pub apply_to: Vec<crate::models::FirewallResource>,
}

impl ApplyToResourcesRequest {
    /// Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources
    pub fn new(apply_to: Vec<crate::models::FirewallResource>) -> ApplyToResourcesRequest {
        ApplyToResourcesRequest {
            apply_to,
        }
    }
}


