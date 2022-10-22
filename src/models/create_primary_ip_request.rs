/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreatePrimaryIpRequest : Request for POST https://api.hetzner.cloud/v1/primary_ips

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePrimaryIpRequest {
    /// ID of the resource the Primary IP should be assigned to. Omitted if it should not be assigned.
    #[serde(rename = "assignee_id", skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<i32>,
    /// Resource type the Primary IP can be assigned to
    #[serde(rename = "assignee_type")]
    pub assignee_type: AssigneeType,
    /// Delete the Primary IP when the Server it is assigned to is deleted. If omitted defaults to `false`.
    #[serde(rename = "auto_delete", skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// ID or name of Datacenter the Primary IP will be bound to. Needs to be omitted if `assignee_id` is passed.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: crate::models::IpType,
}

impl CreatePrimaryIpRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/primary_ips
    pub fn new(
        assignee_type: AssigneeType,
        name: String,
        _type: crate::models::IpType,
    ) -> CreatePrimaryIpRequest {
        CreatePrimaryIpRequest {
            assignee_id: None,
            assignee_type,
            auto_delete: None,
            datacenter: None,
            labels: None,
            name,
            _type,
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
