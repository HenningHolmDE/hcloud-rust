/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ChangeIpRangeOfNetworkRequest : Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeIpRangeOfNetworkRequest {
    /// IP range of the [Network](#networks).  Uses CIDR notation.  Must span all included subnets. Must be one of the private IPv4 ranges of RFC1918.  Minimum network size is /24. We highly recommend that you pick a larger [Network](#networks) with a /16 netmask.
    #[serde(rename = "ip_range")]
    pub ip_range: String,
}

impl ChangeIpRangeOfNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
    pub fn new(ip_range: String) -> ChangeIpRangeOfNetworkRequest {
        ChangeIpRangeOfNetworkRequest { ip_range }
    }
}
