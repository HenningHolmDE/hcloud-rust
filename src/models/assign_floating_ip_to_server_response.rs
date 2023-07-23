/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssignFloatingIpToServerResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssignFloatingIpToServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AssignFloatingIpToServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign
    pub fn new(action: crate::models::Action) -> AssignFloatingIpToServerResponse {
        AssignFloatingIpToServerResponse {
            action: Box::new(action),
        }
    }
}
