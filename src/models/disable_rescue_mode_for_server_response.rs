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

/// DisableRescueModeForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_rescue
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableRescueModeForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DisableRescueModeForServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_rescue
    pub fn new(action: models::Action) -> DisableRescueModeForServerResponse {
        DisableRescueModeForServerResponse {
            action: Box::new(action),
        }
    }
}
