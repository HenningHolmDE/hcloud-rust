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

/// RemoveTargetResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveTargetResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl RemoveTargetResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
    pub fn new(action: models::Action) -> RemoveTargetResponse {
        RemoveTargetResponse {
            action: Box::new(action),
        }
    }
}
