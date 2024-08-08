/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ListPlacementgroupsResponse : Response to GET https://api.hetzner.cloud/v1/placement_groups
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPlacementgroupsResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::Meta>>,
    #[serde(rename = "placement_groups")]
    pub placement_groups: Vec<models::PlacementGroup>,
}

impl ListPlacementgroupsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/placement_groups
    pub fn new(placement_groups: Vec<models::PlacementGroup>) -> ListPlacementgroupsResponse {
        ListPlacementgroupsResponse {
            meta: None,
            placement_groups,
        }
    }
}
