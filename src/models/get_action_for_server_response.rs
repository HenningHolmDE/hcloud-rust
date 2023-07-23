/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}/actions/{action_id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForServerResponse {
        GetActionForServerResponse {
            action: Box::new(action),
        }
    }
}
