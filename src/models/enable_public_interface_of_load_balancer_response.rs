/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EnablePublicInterfaceOfLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/enable_public_interface

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnablePublicInterfaceOfLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl EnablePublicInterfaceOfLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/enable_public_interface
    pub fn new(action: crate::models::Action) -> EnablePublicInterfaceOfLoadBalancerResponse {
        EnablePublicInterfaceOfLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
