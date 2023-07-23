/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddSubnetToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_subnet

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddSubnetToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AddSubnetToNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_subnet
    pub fn new(action: crate::models::Action) -> AddSubnetToNetworkResponse {
        AddSubnetToNetworkResponse {
            action: Box::new(action),
        }
    }
}
