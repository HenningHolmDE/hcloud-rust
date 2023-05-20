/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForLoadBalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForLoadBalancerResponse {
        GetActionForLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
