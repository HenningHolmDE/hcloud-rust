/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListPlacementgroupsResponse : Response to GET https://api.hetzner.cloud/v1/placement_groups

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPlacementgroupsResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
    #[serde(rename = "placement_groups")]
    pub placement_groups: Vec<crate::models::PlacementGroup>,
}

impl ListPlacementgroupsResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/placement_groups
    pub fn new(
        placement_groups: Vec<crate::models::PlacementGroup>,
    ) -> ListPlacementgroupsResponse {
        ListPlacementgroupsResponse {
            meta: None,
            placement_groups,
        }
    }
}
