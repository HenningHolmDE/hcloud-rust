/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Ipv6 : IPv6 network assigned to this Server and its reverse DNS entry

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Ipv6 {
    /// If the IP is blocked by our anti abuse dept
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Reverse DNS PTR entries for the IPv6 addresses of this Server, `null` by default
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Option<Vec<crate::models::DnsPtr>>,
    /// ID of the Resource
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// IP address (v6) of this Server
    #[serde(rename = "ip")]
    pub ip: String,
}

impl Ipv6 {
    #![allow(clippy::too_many_arguments)]
    /// IPv6 network assigned to this Server and its reverse DNS entry
    pub fn new(blocked: bool, dns_ptr: Option<Vec<crate::models::DnsPtr>>, ip: String) -> Ipv6 {
        Ipv6 {
            blocked,
            dns_ptr,
            id: None,
            ip,
        }
    }
}
