/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Rule : Rule of a firewall.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// Select traffic direction on which rule should be applied. Use `source_ips` for direction `in` and `destination_ips` for direction `out`.
    #[serde(rename = "direction")]
    pub direction: Direction,
    /// List of permitted IPv4/IPv6 addresses in CIDR notation. Use `0.0.0.0/0` to allow all IPv4 addresses and `::/0` to allow all IPv6 addresses. You can specify 100 CIDRs at most.
    #[serde(rename = "source_ips", skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,
    /// List of permitted IPv4/IPv6 addresses in CIDR notation. Use `0.0.0.0/0` to allow all IPv4 addresses and `::/0` to allow all IPv6 addresses. You can specify 100 CIDRs at most.
    #[serde(rename = "destination_ips", skip_serializing_if = "Option::is_none")]
    pub destination_ips: Option<Vec<String>>,
    /// Type of traffic to allow
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// Port or port range to which traffic will be allowed, only applicable for protocols TCP and UDP. A port range can be specified by separating two ports with a dash, e.g `1024-5000`.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

impl Rule {
    /// Rule of a firewall.
    pub fn new(direction: Direction, protocol: Protocol) -> Rule {
        Rule {
            direction,
            source_ips: None,
            destination_ips: None,
            protocol,
            port: None,
        }
    }
}

/// Select traffic direction on which rule should be applied. Use `source_ips` for direction `in` and `destination_ips` for direction `out`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "in")]
    _In,
    #[serde(rename = "out")]
    Out,
}
/// Type of traffic to allow
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

