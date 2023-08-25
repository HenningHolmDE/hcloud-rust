/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteRouteFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteRouteFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DeleteRouteFromNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route
    pub fn new(action: crate::models::Action) -> DeleteRouteFromNetworkResponse {
        DeleteRouteFromNetworkResponse {
            action: Box::new(action),
        }
    }
}
