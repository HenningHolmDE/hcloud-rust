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
pub struct FloatingIp {
    /// Whether the IP is blocked
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Description of the Resource
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Array of reverse DNS entries
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Vec<crate::models::DnsPtr>,
    #[serde(rename = "home_location")]
    pub home_location: Box<crate::models::Location>,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i64,
    /// IP address
    #[serde(rename = "ip")]
    pub ip: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// ID of the Server the Floating IP is assigned to, null if it is not assigned at all
    #[serde(rename = "server", deserialize_with = "Option::deserialize")]
    pub server: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: crate::models::IpType,
}

impl FloatingIp {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        blocked: bool,
        created: String,
        description: Option<String>,
        dns_ptr: Vec<crate::models::DnsPtr>,
        home_location: crate::models::Location,
        id: i64,
        ip: String,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        protection: crate::models::Protection,
        server: Option<i64>,
        r#type: crate::models::IpType,
    ) -> FloatingIp {
        FloatingIp {
            blocked,
            created,
            description,
            dns_ptr,
            home_location: Box::new(home_location),
            id,
            ip,
            labels,
            name,
            protection: Box::new(protection),
            server,
            r#type,
        }
    }
}
