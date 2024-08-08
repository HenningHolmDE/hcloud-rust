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

/// GetActionForNetworkResponse : Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl GetActionForNetworkResponse {
    /// Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}
    pub fn new(action: models::Action) -> GetActionForNetworkResponse {
        GetActionForNetworkResponse {
            action: Box::new(action),
        }
    }
}
