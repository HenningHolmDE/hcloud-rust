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

/// Rule : Rule of a firewall.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// Description of the Rule
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// List of permitted IPv4/IPv6 addresses for outgoing traffic (`direction` set to `out`) in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR blocks at most.  The CIDR blocks may refer to networks (with empty host bits) or single hosts. For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`, and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`. Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.
    #[serde(rename = "destination_ips", skip_serializing_if = "Option::is_none")]
    pub destination_ips: Option<Vec<String>>,
    /// Select traffic direction on which rule should be applied. Use `source_ips` for direction `in` and `destination_ips` for direction `out`.
    #[serde(rename = "direction")]
    pub direction: Direction,
    /// Port or port range to which traffic will be allowed, only applicable for protocols TCP and UDP. A port range can be specified by separating two ports with a dash, e.g `1024-5000`.
    #[serde(
        rename = "port",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub port: Option<Option<String>>,
    /// Type of traffic to allow
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// List of permitted IPv4/IPv6 addresses for incoming traffic (`direction` set to `in`) in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR blocks at most.  The CIDR blocks may refer to networks (with empty host bits) or single hosts. For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`, and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`. Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.
    #[serde(rename = "source_ips", skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,
}

impl Rule {
    /// Rule of a firewall.
    pub fn new(direction: Direction, protocol: Protocol) -> Rule {
        Rule {
            description: None,
            destination_ips: None,
            direction,
            port: None,
            protocol,
            source_ips: None,
        }
    }
}
/// Select traffic direction on which rule should be applied. Use `source_ips` for direction `in` and `destination_ips` for direction `out`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::In
    }
}
/// Type of traffic to allow
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "esp")]
    Esp,
    #[serde(rename = "gre")]
    Gre,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Esp
    }
}
