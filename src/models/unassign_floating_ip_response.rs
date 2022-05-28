/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UnassignFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/unassign

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnassignFloatingIpResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl UnassignFloatingIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/unassign
    pub fn new(action: crate::models::Action) -> UnassignFloatingIpResponse {
        UnassignFloatingIpResponse {
            action: Box::new(action),
        }
    }
}
