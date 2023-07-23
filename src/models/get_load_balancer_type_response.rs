/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetLoadBalancerTypeResponse : Response to GET https://api.hetzner.cloud/v1/load_balancer_types/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetLoadBalancerTypeResponse {
    #[serde(rename = "load_balancer_type", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<Box<crate::models::LoadBalancerType>>,
}

impl GetLoadBalancerTypeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/load_balancer_types/{id}
    pub fn new() -> GetLoadBalancerTypeResponse {
        GetLoadBalancerTypeResponse {
            load_balancer_type: None,
        }
    }
}
