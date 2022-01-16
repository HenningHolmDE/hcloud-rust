/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForFloatingIpRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForFloatingIpRequest {
    /// Hostname to set as a reverse DNS PTR entry, will reset to original default value if `null`
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Option<String>,
    /// IP address for which to set the reverse DNS entry
    #[serde(rename = "ip")]
    pub ip: String,
}

impl ChangeReverseDnsEntryForFloatingIpRequest {
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr
    pub fn new(dns_ptr: Option<String>, ip: String) -> ChangeReverseDnsEntryForFloatingIpRequest {
        ChangeReverseDnsEntryForFloatingIpRequest {
            dns_ptr,
            ip,
        }
    }
}


