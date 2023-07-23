/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteServerResponse : Response to DELETE https://api.hetzner.cloud/v1/servers/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteServerResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl DeleteServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to DELETE https://api.hetzner.cloud/v1/servers/{id}
    pub fn new() -> DeleteServerResponse {
        DeleteServerResponse { action: None }
    }
}
