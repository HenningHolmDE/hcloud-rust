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

/// RemoveFromResourcesRequest : Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/remove_from_resources
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveFromResourcesRequest {
    /// Resources the Firewall should be removed from
    #[serde(rename = "remove_from")]
    pub remove_from: Vec<models::FirewallResource>,
}

impl RemoveFromResourcesRequest {
    /// Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/remove_from_resources
    pub fn new(remove_from: Vec<models::FirewallResource>) -> RemoveFromResourcesRequest {
        RemoveFromResourcesRequest { remove_from }
    }
}
