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

/// GetLoadBalancerTypeResponse : Response to GET https://api.hetzner.cloud/v1/load_balancer_types/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLoadBalancerTypeResponse {
    #[serde(rename = "load_balancer_type", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<Box<models::LoadBalancerType>>,
}

impl GetLoadBalancerTypeResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancer_types/{id}
    pub fn new() -> GetLoadBalancerTypeResponse {
        GetLoadBalancerTypeResponse {
            load_balancer_type: None,
        }
    }
}
