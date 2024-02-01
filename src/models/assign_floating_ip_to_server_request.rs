/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssignFloatingIpToServerRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssignFloatingIpToServerRequest {
    /// ID of the Server the Floating IP shall be assigned to
    #[serde(rename = "server")]
    pub server: i64,
}

impl AssignFloatingIpToServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign
    pub fn new(server: i64) -> AssignFloatingIpToServerRequest {
        AssignFloatingIpToServerRequest { server }
    }
}
