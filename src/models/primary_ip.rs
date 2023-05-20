/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrimaryIp {
    /// ID of the resource the Primary IP is assigned to, null if it is not assigned at all
    #[serde(rename = "assignee_id", deserialize_with = "Option::deserialize")]
    pub assignee_id: Option<i64>,
    /// Resource type the Primary IP can be assigned to
    #[serde(rename = "assignee_type")]
    pub assignee_type: AssigneeType,
    /// Delete this Primary IP when the resource it is assigned to is deleted
    #[serde(rename = "auto_delete")]
    pub auto_delete: bool,
    /// Whether the IP is blocked
    #[serde(rename = "blocked")]
    pub blocked: bool,
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "datacenter")]
    pub datacenter: Box<crate::models::Datacenter>,
    /// Array of reverse DNS entries
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Vec<crate::models::DnsPtr>,
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
    #[serde(rename = "type")]
    pub r#type: crate::models::IpType,
}

impl PrimaryIp {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        assignee_id: Option<i64>,
        assignee_type: AssigneeType,
        auto_delete: bool,
        blocked: bool,
        created: String,
        datacenter: crate::models::Datacenter,
        dns_ptr: Vec<crate::models::DnsPtr>,
        id: i64,
        ip: String,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        protection: crate::models::Protection,
        r#type: crate::models::IpType,
    ) -> PrimaryIp {
        PrimaryIp {
            assignee_id,
            assignee_type,
            auto_delete,
            blocked,
            created,
            datacenter: Box::new(datacenter),
            dns_ptr,
            id,
            ip,
            labels,
            name,
            protection: Box::new(protection),
            r#type,
        }
    }
}

/// Resource type the Primary IP can be assigned to
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "server")]
    Server,
}

impl Default for AssigneeType {
    fn default() -> AssigneeType {
        Self::Server
    }
}
