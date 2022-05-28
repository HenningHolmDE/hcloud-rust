/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Ipv4 : IP address (v4) and its reverse DNS entry of this Server

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Ipv4 {
    /// If the IP is blocked by our anti abuse dept
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Reverse DNS PTR entry for the IPv4 addresses of this Server
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: String,
    /// ID of the Resource
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// IP address (v4) of this Server
    #[serde(rename = "ip")]
    pub ip: String,
}

impl Ipv4 {
    #![allow(clippy::too_many_arguments)]
    /// IP address (v4) and its reverse DNS entry of this Server
    pub fn new(blocked: bool, dns_ptr: String, ip: String) -> Ipv4 {
        Ipv4 {
            blocked,
            dns_ptr,
            id: None,
            ip,
        }
    }
}
