/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Network {
    /// Point in time when the Network was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Indicates if the routes from this network should be exposed to the vSwitch connection. The exposing only takes effect if a vSwitch connection is active.  Currently the default route 0.0.0.0/0 is not exposed to the vSwitch connection. We are aware of the issue and are working on a solution.
    #[serde(rename = "expose_routes_to_vswitch")]
    pub expose_routes_to_vswitch: bool,
    /// ID of the Network
    #[serde(rename = "id")]
    pub id: i64,
    /// IPv4 prefix of the whole Network
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Array of IDs of Load Balancers attached to this Network
    #[serde(rename = "load_balancers", skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<i64>>,
    /// Name of the Network
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// Array of routes set in this Network
    #[serde(rename = "routes")]
    pub routes: Vec<crate::models::Route>,
    /// Array of IDs of Servers attached to this Network
    #[serde(rename = "servers")]
    pub servers: Vec<i64>,
    /// Array subnets allocated in this Network
    #[serde(rename = "subnets")]
    pub subnets: Vec<crate::models::SubnetWithGateway>,
}

impl Network {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        created: String,
        expose_routes_to_vswitch: bool,
        id: i64,
        ip_range: String,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        protection: crate::models::Protection,
        routes: Vec<crate::models::Route>,
        servers: Vec<i64>,
        subnets: Vec<crate::models::SubnetWithGateway>,
    ) -> Network {
        Network {
            created,
            expose_routes_to_vswitch,
            id,
            ip_range,
            labels,
            load_balancers: None,
            name,
            protection: Box::new(protection),
            routes,
            servers,
            subnets,
        }
    }
}
