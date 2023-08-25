/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubnetWithGateway {
    /// Gateway for Servers attached to this subnet. For subnets of type Server this is always the first IP of the network IP range.
    #[serde(rename = "gateway")]
    pub gateway: String,
    /// Range to allocate IPs from. Must be a Subnet of the ip_range of the parent network object and must not overlap with any other subnets or with any destinations in routes. Minimum Network size is /30. We suggest that you pick a bigger Network with a /24 netmask.
    #[serde(rename = "ip_range", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// Name of Network zone. The Location object contains the `network_zone` property each Location belongs to.
    #[serde(rename = "network_zone")]
    pub network_zone: String,
    /// Type of Subnetwork
    #[serde(rename = "type")]
    pub r#type: Type,
    /// ID of the robot vSwitch if the subnet is of type vswitch.
    #[serde(
        rename = "vswitch_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub vswitch_id: Option<Option<i64>>,
}

impl SubnetWithGateway {
    #![allow(clippy::too_many_arguments)]
    pub fn new(gateway: String, network_zone: String, r#type: Type) -> SubnetWithGateway {
        SubnetWithGateway {
            gateway,
            ip_range: None,
            network_zone,
            r#type,
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
