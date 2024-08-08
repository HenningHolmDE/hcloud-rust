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

/// DetachLoadBalancerFromNetworkRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachLoadBalancerFromNetworkRequest {
    /// ID of an existing network to detach the Load Balancer from
    #[serde(rename = "network")]
    pub network: i64,
}

impl DetachLoadBalancerFromNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network
    pub fn new(network: i64) -> DetachLoadBalancerFromNetworkRequest {
        DetachLoadBalancerFromNetworkRequest { network }
    }
}
