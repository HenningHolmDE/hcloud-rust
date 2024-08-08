/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DetachIsoFromServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_iso
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachIsoFromServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DetachIsoFromServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_iso
    pub fn new(action: models::Action) -> DetachIsoFromServerResponse {
        DetachIsoFromServerResponse {
            action: Box::new(action),
        }
    }
}
