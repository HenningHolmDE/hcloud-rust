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

/// AssignPrimaryIpToResourceRequest : Request for POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/assign
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignPrimaryIpToResourceRequest {
    /// ID of a resource of type `assignee_type`
    #[serde(rename = "assignee_id")]
    pub assignee_id: i64,
    /// Type of resource assigning the Primary IP to
    #[serde(rename = "assignee_type")]
    pub assignee_type: AssigneeType,
}

impl AssignPrimaryIpToResourceRequest {
    /// Request for POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/assign
    pub fn new(assignee_id: i64, assignee_type: AssigneeType) -> AssignPrimaryIpToResourceRequest {
        AssignPrimaryIpToResourceRequest {
            assignee_id,
            assignee_type,
        }
    }
}
/// Type of resource assigning the Primary IP to
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
