/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteSubnetFromNetworkRequest : Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSubnetFromNetworkRequest {
    /// IP range of subnet to delete
    #[serde(rename = "ip_range")]
    pub ip_range: String,
}

impl DeleteSubnetFromNetworkRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet
    pub fn new(ip_range: String) -> DeleteSubnetFromNetworkRequest {
        DeleteSubnetFromNetworkRequest { ip_range }
    }
}
