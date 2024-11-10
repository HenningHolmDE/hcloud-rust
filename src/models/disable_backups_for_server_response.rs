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

/// DisableBackupsForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableBackupsForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DisableBackupsForServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup
    pub fn new(action: models::Action) -> DisableBackupsForServerResponse {
        DisableBackupsForServerResponse {
            action: Box::new(action),
        }
    }
}
