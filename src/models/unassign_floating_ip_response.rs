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

/// UnassignFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/unassign
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnassignFloatingIpResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl UnassignFloatingIpResponse {
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/unassign
    pub fn new(action: models::Action) -> UnassignFloatingIpResponse {
        UnassignFloatingIpResponse {
            action: Box::new(action),
        }
    }
}
