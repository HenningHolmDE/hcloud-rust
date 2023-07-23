/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DnsPtr {
    /// DNS pointer for the specific IP address
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: String,
    /// Single IPv4 or IPv6 address | Single IPv6 address of this Server for which the reverse DNS entry has been set up
    #[serde(rename = "ip")]
    pub ip: String,
}

impl DnsPtr {
    #![allow(clippy::too_many_arguments)]
    pub fn new(dns_ptr: String, ip: String) -> DnsPtr {
        DnsPtr { dns_ptr, ip }
    }
}
