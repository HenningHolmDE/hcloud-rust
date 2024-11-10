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

/// GetActionForServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}/actions/{action_id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl GetActionForServerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}/actions/{action_id}
    pub fn new(action: models::Action) -> GetActionForServerResponse {
        GetActionForServerResponse {
            action: Box::new(action),
        }
    }
}
