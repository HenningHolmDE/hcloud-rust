/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ChangeNetworkProtectionResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeNetworkProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeNetworkProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection
    pub fn new(action: models::Action) -> ChangeNetworkProtectionResponse {
        ChangeNetworkProtectionResponse {
            action: Box::new(action),
        }
    }
}
