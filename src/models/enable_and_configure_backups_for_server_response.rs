/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EnableAndConfigureBackupsForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_backup
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableAndConfigureBackupsForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl EnableAndConfigureBackupsForServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_backup
    pub fn new(action: models::Action) -> EnableAndConfigureBackupsForServerResponse {
        EnableAndConfigureBackupsForServerResponse {
            action: Box::new(action),
        }
    }
}
