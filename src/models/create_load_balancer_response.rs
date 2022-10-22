/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
    #[serde(rename = "load_balancer")]
    pub load_balancer: Box<crate::models::LoadBalancer>,
}

impl CreateLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers
    pub fn new(
        action: crate::models::Action,
        load_balancer: crate::models::LoadBalancer,
    ) -> CreateLoadBalancerResponse {
        CreateLoadBalancerResponse {
            action: Box::new(action),
            load_balancer: Box::new(load_balancer),
        }
    }
}
