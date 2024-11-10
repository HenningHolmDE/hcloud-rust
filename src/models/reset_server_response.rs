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

/// ResetServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ResetServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset
    pub fn new(action: models::Action) -> ResetServerResponse {
        ResetServerResponse {
            action: Box::new(action),
        }
    }
}
