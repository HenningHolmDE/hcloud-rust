/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForNetworkResponse : Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForNetworkResponse {
        GetActionForNetworkResponse {
            action: Box::new(action),
        }
    }
}
