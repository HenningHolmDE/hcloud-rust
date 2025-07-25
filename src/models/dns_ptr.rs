/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DnsPtr : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr | Request for POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/change_dns_ptr
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsPtr {
    /// Domain Name to point to.  PTR record content used for reverse DNS.  | DNS pointer for the specific IP address.
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: String,
    /// Single IPv4 or IPv6 address to create pointer for.  | Single IPv6 address of this Server for which the reverse DNS entry has been set up.
    #[serde(rename = "ip")]
    pub ip: String,
}

impl DnsPtr {
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr | Request for POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/change_dns_ptr
    pub fn new(dns_ptr: String, ip: String) -> DnsPtr {
        DnsPtr { dns_ptr, ip }
    }
}
