/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeTypeOfServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeTypeOfServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeTypeOfServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type
    pub fn new(action: crate::models::Action) -> ChangeTypeOfServerResponse {
        ChangeTypeOfServerResponse {
            action: Box::new(action),
        }
    }
}
