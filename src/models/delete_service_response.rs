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

/// DeleteServiceResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteServiceResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl DeleteServiceResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service
    pub fn new(action: models::Action) -> DeleteServiceResponse {
        DeleteServiceResponse {
            action: Box::new(action),
        }
    }
}
