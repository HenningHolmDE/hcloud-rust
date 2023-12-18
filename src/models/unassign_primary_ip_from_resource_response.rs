/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UnassignPrimaryIpFromResourceResponse : Response to POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/unassign

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnassignPrimaryIpFromResourceResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl UnassignPrimaryIpFromResourceResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/unassign
    pub fn new(action: crate::models::Action) -> UnassignPrimaryIpFromResourceResponse {
        UnassignPrimaryIpFromResourceResponse {
            action: Box::new(action),
        }
    }
}
