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

/// GetActionForLoadBalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl GetActionForLoadBalancerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions/{action_id}
    pub fn new(action: models::Action) -> GetActionForLoadBalancerResponse {
        GetActionForLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
