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

/// LoadBalancerTargetIp : IP targets where the traffic should be routed to. It is only possible to use the (Public or vSwitch) IPs of Hetzner Online Root Servers belonging to the project owner. IPs belonging to other users are blocked. Additionally IPs belonging to services provided by Hetzner Cloud (Servers, Load Balancers, ...) are blocked as well. Only present for target type \"ip\".
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerTargetIp {
    /// IP of a server that belongs to the same customer (public IPv4/IPv6) or private IP in a Subnetwork type vswitch.
    #[serde(rename = "ip")]
    pub ip: String,
}

impl LoadBalancerTargetIp {
    /// IP targets where the traffic should be routed to. It is only possible to use the (Public or vSwitch) IPs of Hetzner Online Root Servers belonging to the project owner. IPs belonging to other users are blocked. Additionally IPs belonging to services provided by Hetzner Cloud (Servers, Load Balancers, ...) are blocked as well. Only present for target type \"ip\".
    pub fn new(ip: String) -> LoadBalancerTargetIp {
        LoadBalancerTargetIp { ip }
    }
}
