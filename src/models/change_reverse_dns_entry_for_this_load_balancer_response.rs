/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForThisLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForThisLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeReverseDnsEntryForThisLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr
    pub fn new(action: crate::models::Action) -> ChangeReverseDnsEntryForThisLoadBalancerResponse {
        ChangeReverseDnsEntryForThisLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
