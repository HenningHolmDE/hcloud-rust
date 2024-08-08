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

/// GetActionForVolumeResponse : Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions/{action_id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForVolumeResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl GetActionForVolumeResponse {
    /// Response to GET https://api.hetzner.cloud/v1/volumes/{id}/actions/{action_id}
    pub fn new(action: models::Action) -> GetActionForVolumeResponse {
        GetActionForVolumeResponse {
            action: Box::new(action),
        }
    }
}
