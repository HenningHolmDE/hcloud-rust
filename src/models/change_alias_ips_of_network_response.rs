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

/// ChangeAliasIpsOfNetworkResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_alias_ips
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAliasIpsOfNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeAliasIpsOfNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_alias_ips
    pub fn new(action: models::Action) -> ChangeAliasIpsOfNetworkResponse {
        ChangeAliasIpsOfNetworkResponse {
            action: Box::new(action),
        }
    }
}
