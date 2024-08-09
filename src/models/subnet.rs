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

/// Subnet : Subnets divide the ip_range from the parent Network object into multiple Subnetworks that you can use for different specific purposes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    /// IP range in CIDR block notation of the whole subnetwork.  Must be a subnet of the parent Network `ip_range`.  Must not overlap with any other subnets or with any destinations in routes.  Minimum network size is /30. We highly recommend that you pick a larger network with a /24 netmask.
    #[serde(rename = "ip_range", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// Name of Network zone.  The Location object contains the `network_zone` property each Location belongs to.  | Name of Network Zone.  The Location object contains the `network_zone` it belongs to.
    #[serde(rename = "network_zone")]
    pub network_zone: String,
    /// Type of Subnetwork.  - `cloud` - Used to connect cloud servers and load balancers. - `server` - Same as the `cloud` type. **Deprecated**, use the `cloud` type instead. - `vswitch` - Used to [connect cloud servers and load balancers with dedicated servers](https://docs.hetzner.com/cloud/networks/connect-dedi-vswitch).
    #[serde(rename = "type")]
    pub r#type: Type,
    /// ID of the robot vSwitch.  Must only be supplied for subnets of type vswitch.  | ID of the robot vSwitch.  Must be supplied if the Subnet is of type `vswitch`.
    #[serde(rename = "vswitch_id", skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<i64>,
}

impl Subnet {
    /// Subnets divide the ip_range from the parent Network object into multiple Subnetworks that you can use for different specific purposes.
    pub fn new(network_zone: String, r#type: Type) -> Subnet {
        Subnet {
            ip_range: None,
            network_zone,
            r#type,
            vswitch_id: None,
        }
    }
}
/// Type of Subnetwork.  - `cloud` - Used to connect cloud servers and load balancers. - `server` - Same as the `cloud` type. **Deprecated**, use the `cloud` type instead. - `vswitch` - Used to [connect cloud servers and load balancers with dedicated servers](https://docs.hetzner.com/cloud/networks/connect-dedi-vswitch).
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
