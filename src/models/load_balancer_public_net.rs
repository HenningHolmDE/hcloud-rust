/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LoadBalancerPublicNet : Public network information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerPublicNet {
    /// Public Interface enabled or not
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "ipv4")]
    pub ipv4: Box<models::LoadBalancerPublicNetIpv4>,
    #[serde(rename = "ipv6")]
    pub ipv6: Box<models::LoadBalancerPublicNetIpv6>,
}

impl LoadBalancerPublicNet {
    /// Public network information
    pub fn new(
        enabled: bool,
        ipv4: models::LoadBalancerPublicNetIpv4,
        ipv6: models::LoadBalancerPublicNetIpv6,
    ) -> LoadBalancerPublicNet {
        LoadBalancerPublicNet {
            enabled,
            ipv4: Box::new(ipv4),
            ipv6: Box::new(ipv6),
        }
    }
}
