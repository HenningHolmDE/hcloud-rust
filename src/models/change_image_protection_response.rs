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

/// ChangeImageProtectionResponse : Response to POST https://api.hetzner.cloud/v1/images/{id}/actions/change_protection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeImageProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeImageProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/images/{id}/actions/change_protection
    pub fn new(action: models::Action) -> ChangeImageProtectionResponse {
        ChangeImageProtectionResponse {
            action: Box::new(action),
        }
    }
}
