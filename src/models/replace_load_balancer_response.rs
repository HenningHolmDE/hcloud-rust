/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceLoadBalancerResponse : Response to PUT https://api.hetzner.cloud/v1/load_balancers/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceLoadBalancerResponse {
    #[serde(rename = "load_balancer")]
    pub load_balancer: Box<crate::models::LoadBalancer>,
}

impl ReplaceLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/load_balancers/{id}
    pub fn new(load_balancer: crate::models::LoadBalancer) -> ReplaceLoadBalancerResponse {
        ReplaceLoadBalancerResponse {
            load_balancer: Box::new(load_balancer),
        }
    }
}
