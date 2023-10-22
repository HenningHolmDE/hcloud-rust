/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.18.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeIpRangeOfNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeIpRangeOfNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeIpRangeOfNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
    pub fn new(action: crate::models::Action) -> ChangeIpRangeOfNetworkResponse {
        ChangeIpRangeOfNetworkResponse {
            action: Box::new(action),
        }
    }
}
