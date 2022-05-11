/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachLoadBalancerToNetworkRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachLoadBalancerToNetworkRequest {
    /// IP to request to be assigned to this Load Balancer; if you do not provide this then you will be auto assigned an IP address
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// ID of an existing network to attach the Load Balancer to
    #[serde(rename = "network")]
    pub network: i32,
}

impl AttachLoadBalancerToNetworkRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network
    pub fn new(network: i32) -> AttachLoadBalancerToNetworkRequest {
        AttachLoadBalancerToNetworkRequest { ip: None, network }
    }
}
