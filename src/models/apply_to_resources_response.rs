/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ApplyToResourcesResponse : Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplyToResourcesResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ApplyToResourcesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources
    pub fn new(actions: Vec<crate::models::Action>) -> ApplyToResourcesResponse {
        ApplyToResourcesResponse {
            actions,
            meta: None,
        }
    }
}
