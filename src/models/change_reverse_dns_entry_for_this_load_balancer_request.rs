/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForThisLoadBalancerRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForThisLoadBalancerRequest {
    /// Hostname to set as a reverse DNS PTR entry
    #[serde(rename = "dns_ptr", deserialize_with = "Option::deserialize")]
    pub dns_ptr: Option<String>,
    /// Public IP address for which the reverse DNS entry should be set
    #[serde(rename = "ip")]
    pub ip: String,
}

impl ChangeReverseDnsEntryForThisLoadBalancerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr
    pub fn new(
        dns_ptr: Option<String>,
        ip: String,
    ) -> ChangeReverseDnsEntryForThisLoadBalancerRequest {
        ChangeReverseDnsEntryForThisLoadBalancerRequest { dns_ptr, ip }
    }
}
