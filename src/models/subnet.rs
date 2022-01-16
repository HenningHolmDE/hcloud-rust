/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Subnet : Subnets divide the ip_range from the parent Network object into multiple Subnetworks that you can use for different specific purposes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subnet {
    /// Range to allocate IPs from. Must be a Subnet of the ip_range of the parent network object and must not overlap with any other subnets or with any destinations in routes. If the Subnet is of type vSwitch, it also can not overlap with any gateway in routes. Minimum Network size is /30. We suggest that you pick a bigger Network with a /24 netmask.
    #[serde(rename = "ip_range", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// Name of Network zone. Currently eu-central is the only available zone.
    #[serde(rename = "network_zone")]
    pub network_zone: String,
    /// Type of Subnetwork
    #[serde(rename = "type")]
    pub _type: Type,
    /// ID of the robot vSwitch. Must be supplied if the subnet is of type vswitch.
    #[serde(rename = "vswitch_id", skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<i32>,
}

impl Subnet {
    /// Subnets divide the ip_range from the parent Network object into multiple Subnetworks that you can use for different specific purposes.
    pub fn new(network_zone: String, _type: Type) -> Subnet {
        Subnet {
            ip_range: None,
            network_zone,
            _type,
            vswitch_id: None,
        }
    }
}

/// Type of Subnetwork
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "vswitch")]
    Vswitch,
}

impl Default for Type {
    fn default() -> Type {
        Self::Cloud
    }
}

