/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ChangeReverseDnsEntryForThisLoadBalancerRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForThisLoadBalancerRequest {
    /// Hostname to set as a reverse DNS PTR entry
    #[serde(rename = "dns_ptr", deserialize_with = "Option::deserialize")]
    pub dns_ptr: Option<String>,
    /// Public IP address for which the reverse DNS entry should be set
    #[serde(rename = "ip")]
    pub ip: String,
}

impl ChangeReverseDnsEntryForThisLoadBalancerRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_dns_ptr
    pub fn new(
        dns_ptr: Option<String>,
        ip: String,
    ) -> ChangeReverseDnsEntryForThisLoadBalancerRequest {
        ChangeReverseDnsEntryForThisLoadBalancerRequest { dns_ptr, ip }
    }
}
