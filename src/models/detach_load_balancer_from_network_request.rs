/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DetachLoadBalancerFromNetworkRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachLoadBalancerFromNetworkRequest {
    /// ID of an existing network to detach the Load Balancer from
    #[serde(rename = "network")]
    pub network: i32,
}

impl DetachLoadBalancerFromNetworkRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network
    pub fn new(network: i32) -> DetachLoadBalancerFromNetworkRequest {
        DetachLoadBalancerFromNetworkRequest { network }
    }
}
