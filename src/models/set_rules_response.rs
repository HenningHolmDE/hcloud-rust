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

/// SetRulesResponse : Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetRulesResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<models::Action>,
}

impl SetRulesResponse {
    /// Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules
    pub fn new(actions: Vec<models::Action>) -> SetRulesResponse {
        SetRulesResponse { actions }
    }
}
