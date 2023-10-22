/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.18.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreatePlacementgroupResponse : Response to POST https://api.hetzner.cloud/v1/placement_groups

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePlacementgroupResponse {
    #[serde(
        rename = "action",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<Option<Box<crate::models::Action>>>,
    #[serde(rename = "placement_group")]
    pub placement_group: Box<crate::models::PlacementGroup>,
}

impl CreatePlacementgroupResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/placement_groups
    pub fn new(placement_group: crate::models::PlacementGroup) -> CreatePlacementgroupResponse {
        CreatePlacementgroupResponse {
            action: None,
            placement_group: Box::new(placement_group),
        }
    }
}
