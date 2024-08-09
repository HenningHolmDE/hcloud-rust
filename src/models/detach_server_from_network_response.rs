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

/// DetachServerFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachServerFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DetachServerFromNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_from_network
    pub fn new(action: models::Action) -> DetachServerFromNetworkResponse {
        DetachServerFromNetworkResponse {
            action: Box::new(action),
        }
    }
}
