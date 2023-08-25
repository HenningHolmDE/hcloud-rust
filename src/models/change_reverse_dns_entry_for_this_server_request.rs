/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForThisServerRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_dns_ptr

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForThisServerRequest {
    /// Hostname to set as a reverse DNS PTR entry, reset to original value if `null`
    #[serde(rename = "dns_ptr", deserialize_with = "Option::deserialize")]
    pub dns_ptr: Option<String>,
    /// Primary IP address for which the reverse DNS entry should be set
    #[serde(rename = "ip")]
    pub ip: String,
}

impl ChangeReverseDnsEntryForThisServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_dns_ptr
    pub fn new(dns_ptr: Option<String>, ip: String) -> ChangeReverseDnsEntryForThisServerRequest {
        ChangeReverseDnsEntryForThisServerRequest { dns_ptr, ip }
    }
}
