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

/// PowerOffServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweroff
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerOffServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl PowerOffServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweroff
    pub fn new(action: models::Action) -> PowerOffServerResponse {
        PowerOffServerResponse {
            action: Box::new(action),
        }
    }
}
