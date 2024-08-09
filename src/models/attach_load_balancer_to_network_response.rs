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

/// AttachLoadBalancerToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachLoadBalancerToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl AttachLoadBalancerToNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network
    pub fn new(action: models::Action) -> AttachLoadBalancerToNetworkResponse {
        AttachLoadBalancerToNetworkResponse {
            action: Box::new(action),
        }
    }
}
