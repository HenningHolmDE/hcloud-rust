/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DetachServerFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachServerFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DetachServerFromNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network
    pub fn new(action: crate::models::Action) -> DetachServerFromNetworkResponse {
        DetachServerFromNetworkResponse {
            action: Box::new(action),
        }
    }
}
