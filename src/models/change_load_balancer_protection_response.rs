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

/// ChangeLoadBalancerProtectionResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeLoadBalancerProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeLoadBalancerProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection
    pub fn new(action: models::Action) -> ChangeLoadBalancerProtectionResponse {
        ChangeLoadBalancerProtectionResponse {
            action: Box::new(action),
        }
    }
}
