/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeAliasIpsOfNetworkRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_alias_ips

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeAliasIpsOfNetworkRequest {
    /// New alias IPs to set for this Server
    #[serde(rename = "alias_ips")]
    pub alias_ips: Vec<String>,
    /// ID of an existing Network already attached to the Server
    #[serde(rename = "network")]
    pub network: i32,
}

impl ChangeAliasIpsOfNetworkRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_alias_ips
    pub fn new(alias_ips: Vec<String>, network: i32) -> ChangeAliasIpsOfNetworkRequest {
        ChangeAliasIpsOfNetworkRequest { alias_ips, network }
    }
}
