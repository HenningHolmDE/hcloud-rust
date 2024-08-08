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

/// CreatePlacementgroupResponse : Response to POST https://api.hetzner.cloud/v1/placement_groups
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlacementgroupResponse {
    #[serde(
        rename = "action",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<Option<Box<models::Action>>>,
    #[serde(rename = "placement_group")]
    pub placement_group: Box<models::PlacementGroup>,
}

impl CreatePlacementgroupResponse {
    /// Response to POST https://api.hetzner.cloud/v1/placement_groups
    pub fn new(placement_group: models::PlacementGroup) -> CreatePlacementgroupResponse {
        CreatePlacementgroupResponse {
            action: None,
            placement_group: Box::new(placement_group),
        }
    }
}
