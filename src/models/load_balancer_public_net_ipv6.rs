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

/// LoadBalancerPublicNetIpv6 : IP address (v6).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerPublicNetIpv6 {
    /// Reverse DNS PTR entry for the IPv6 address of this Load Balancer.
    #[serde(
        rename = "dns_ptr",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_ptr: Option<Option<String>>,
    /// IP address (v6) of this Load Balancer.
    #[serde(
        rename = "ip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip: Option<Option<String>>,
}

impl LoadBalancerPublicNetIpv6 {
    /// IP address (v6).
    pub fn new() -> LoadBalancerPublicNetIpv6 {
        LoadBalancerPublicNetIpv6 {
            dns_ptr: None,
            ip: None,
        }
    }
}
