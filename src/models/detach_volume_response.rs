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

/// DetachVolumeResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/detach
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachVolumeResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DetachVolumeResponse {
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/detach
    pub fn new(action: models::Action) -> DetachVolumeResponse {
        DetachVolumeResponse {
            action: Box::new(action),
        }
    }
}
