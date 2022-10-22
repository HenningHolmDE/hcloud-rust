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
    pub r#type: RHashType,
}

impl LoadBalancerAlgorithm {
    #![allow(clippy::too_many_arguments)]
    /// Algorithm of the Load Balancer | Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_algorithm
    pub fn new(r#type: RHashType) -> LoadBalancerAlgorithm {
        LoadBalancerAlgorithm { r#type }
    }
}

/// Type of the algorithm | Algorithm of the Load Balancer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "least_connections")]
    LeastConnections,
    #[serde(rename = "round_robin")]
    RoundRobin,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LeastConnections
    }
}
