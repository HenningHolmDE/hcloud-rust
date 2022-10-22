/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeLoadBalancerProtectionResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeLoadBalancerProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeLoadBalancerProtectionResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeLoadBalancerProtectionResponse {
        ChangeLoadBalancerProtectionResponse {
            action: Box::new(action),
        }
    }
}
