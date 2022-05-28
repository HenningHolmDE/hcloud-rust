/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForVolumeResponse : Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListActionsForVolumeResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListActionsForVolumeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForVolumeResponse {
        ListActionsForVolumeResponse {
            actions,
            meta: None,
        }
    }
}
