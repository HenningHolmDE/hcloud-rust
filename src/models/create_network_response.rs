/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateNetworkResponse {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<crate::models::Network>>,
}

impl CreateNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/networks
    pub fn new() -> CreateNetworkResponse {
        CreateNetworkResponse { network: None }
    }
}
