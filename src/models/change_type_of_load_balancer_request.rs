/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeTypeOfLoadBalancerRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeTypeOfLoadBalancerRequest {
    /// ID or name of Load Balancer type the Load Balancer should migrate to
    #[serde(rename = "load_balancer_type")]
    pub load_balancer_type: String,
}

impl ChangeTypeOfLoadBalancerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type
    pub fn new(load_balancer_type: String) -> ChangeTypeOfLoadBalancerRequest {
        ChangeTypeOfLoadBalancerRequest { load_balancer_type }
    }
}
