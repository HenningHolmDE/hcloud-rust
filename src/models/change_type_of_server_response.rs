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

/// ChangeTypeOfServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeTypeOfServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeTypeOfServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type
    pub fn new(action: models::Action) -> ChangeTypeOfServerResponse {
        ChangeTypeOfServerResponse {
            action: Box::new(action),
        }
    }
}
