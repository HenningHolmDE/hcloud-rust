/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachServerToNetworkRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachServerToNetworkRequest {
    /// Additional IPs to be assigned to this Server
    #[serde(rename = "alias_ips", skip_serializing_if = "Option::is_none")]
    pub alias_ips: Option<Vec<String>>,
    /// IP to request to be assigned to this Server; if you do not provide this then you will be auto assigned an IP address
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// ID of an existing network to attach the Server to
    #[serde(rename = "network")]
    pub network: i64,
}

impl AttachServerToNetworkRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network
    pub fn new(network: i64) -> AttachServerToNetworkRequest {
        AttachServerToNetworkRequest {
            alias_ips: None,
            ip: None,
            network,
        }
    }
}
