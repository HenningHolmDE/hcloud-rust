/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeLoadBalancerProtectionRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeLoadBalancerProtectionRequest {
    /// If true, prevents the Load Balancer from being deleted
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}

impl ChangeLoadBalancerProtectionRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection
    pub fn new() -> ChangeLoadBalancerProtectionRequest {
        ChangeLoadBalancerProtectionRequest { delete: None }
    }
}
