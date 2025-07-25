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

/// EnablePublicInterfaceOfLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/enable_public_interface
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnablePublicInterfaceOfLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl EnablePublicInterfaceOfLoadBalancerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/enable_public_interface
    pub fn new(action: models::Action) -> EnablePublicInterfaceOfLoadBalancerResponse {
        EnablePublicInterfaceOfLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}
