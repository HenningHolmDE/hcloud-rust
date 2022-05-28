/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplacePlacementgroupResponse : Response to PUT https://api.hetzner.cloud/v1/placement_groups/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplacePlacementgroupResponse {
    #[serde(rename = "placement_group")]
    pub placement_group: Box<crate::models::PlacementGroup>,
}

impl ReplacePlacementgroupResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/placement_groups/{id}
    pub fn new(placement_group: crate::models::PlacementGroup) -> ReplacePlacementgroupResponse {
        ReplacePlacementgroupResponse {
            placement_group: Box::new(placement_group),
        }
    }
}
