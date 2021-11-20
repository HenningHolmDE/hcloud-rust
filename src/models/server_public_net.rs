/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerPublicNet : Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerPublicNet {
    #[serde(rename = "ipv4")]
    pub ipv4: Box<crate::models::Ipv4>,
    #[serde(rename = "ipv6")]
    pub ipv6: Box<crate::models::Ipv6>,
    /// IDs of Floating IPs assigned to this Server
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<i32>,
    /// Firewalls applied to the public network interface of this Server
    #[serde(rename = "firewalls", skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<crate::models::ServerPublicNetFirewall>>,
}

impl ServerPublicNet {
    /// Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`
    pub fn new(ipv4: crate::models::Ipv4, ipv6: crate::models::Ipv6, floating_ips: Vec<i32>) -> ServerPublicNet {
        ServerPublicNet {
            ipv4: Box::new(ipv4),
            ipv6: Box::new(ipv6),
            floating_ips,
            firewalls: None,
        }
    }
}


