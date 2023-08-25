/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeTypeOfLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeTypeOfLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeTypeOfLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type
    pub fn new(action: crate::models::Action) -> ChangeTypeOfLoadBalancerResponse {
        ChangeTypeOfLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
