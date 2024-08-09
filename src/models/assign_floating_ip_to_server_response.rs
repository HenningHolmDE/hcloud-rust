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

/// AssignFloatingIpToServerResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignFloatingIpToServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl AssignFloatingIpToServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign
    pub fn new(action: models::Action) -> AssignFloatingIpToServerResponse {
        AssignFloatingIpToServerResponse {
            action: Box::new(action),
        }
    }
}
