/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerAlgorithm : Algorithm of the Load Balancer | Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_algorithm

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerAlgorithm {
    /// Type of the algorithm | Algorithm of the Load Balancer
    #[serde(rename = "type")]
    pub _type: Type,
}

impl LoadBalancerAlgorithm {
    #![allow(clippy::too_many_arguments)]
    /// Algorithm of the Load Balancer | Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_algorithm
    pub fn new(_type: Type) -> LoadBalancerAlgorithm {
        LoadBalancerAlgorithm { _type }
    }
}

/// Type of the algorithm | Algorithm of the Load Balancer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "least_connections")]
    LeastConnections,
    #[serde(rename = "round_robin")]
    RoundRobin,
}

impl Default for Type {
    fn default() -> Type {
        Self::LeastConnections
    }
}
