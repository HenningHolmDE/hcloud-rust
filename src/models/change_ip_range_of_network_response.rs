/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ChangeIpRangeOfNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeIpRangeOfNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeIpRangeOfNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
    pub fn new(action: models::Action) -> ChangeIpRangeOfNetworkResponse {
        ChangeIpRangeOfNetworkResponse {
            action: Box::new(action),
        }
    }
}
