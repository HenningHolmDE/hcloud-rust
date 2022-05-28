/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerPublicNetIpv4 : IP address (v4)

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerPublicNetIpv4 {
    /// Reverse DNS PTR entry for the IPv4 address of this Load Balancer
    #[serde(rename = "dns_ptr", skip_serializing_if = "Option::is_none")]
    pub dns_ptr: Option<String>,
    /// IP address (v4) of this Load Balancer
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl LoadBalancerPublicNetIpv4 {
    #![allow(clippy::too_many_arguments)]
    /// IP address (v4)
    pub fn new() -> LoadBalancerPublicNetIpv4 {
        LoadBalancerPublicNetIpv4 {
            dns_ptr: None,
            ip: None,
        }
    }
}
