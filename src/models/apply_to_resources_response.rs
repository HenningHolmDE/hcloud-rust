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

/// ApplyToResourcesResponse : Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyToResourcesResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::Meta>>,
}

impl ApplyToResourcesResponse {
    /// Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/apply_to_resources
    pub fn new(actions: Vec<models::Action>) -> ApplyToResourcesResponse {
        ApplyToResourcesResponse {
            actions,
            meta: None,
        }
    }
}
