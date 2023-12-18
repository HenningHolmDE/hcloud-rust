/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteSubnetFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSubnetFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DeleteSubnetFromNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet
    pub fn new(action: crate::models::Action) -> DeleteSubnetFromNetworkResponse {
        DeleteSubnetFromNetworkResponse {
            action: Box::new(action),
        }
    }
}
