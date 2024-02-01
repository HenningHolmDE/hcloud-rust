/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetPlacementgroupResponse : Response to GET https://api.hetzner.cloud/v1/placement_groups/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPlacementgroupResponse {
    #[serde(rename = "placement_group")]
    pub placement_group: Box<crate::models::PlacementGroup>,
}

impl GetPlacementgroupResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/placement_groups/{id}
    pub fn new(placement_group: crate::models::PlacementGroup) -> GetPlacementgroupResponse {
        GetPlacementgroupResponse {
            placement_group: Box::new(placement_group),
        }
    }
}
