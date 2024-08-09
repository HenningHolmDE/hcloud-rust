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

/// ChangeVolumeProtectionResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeVolumeProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeVolumeProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection
    pub fn new(action: models::Action) -> ChangeVolumeProtectionResponse {
        ChangeVolumeProtectionResponse {
            action: Box::new(action),
        }
    }
}
