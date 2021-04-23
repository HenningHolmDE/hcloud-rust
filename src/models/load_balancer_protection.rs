/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerProtection : Protection configuration for the Load Balancer



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerProtection {
    /// If true, prevents the Load Balancer from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl LoadBalancerProtection {
    /// Protection configuration for the Load Balancer
    pub fn new(delete: bool) -> LoadBalancerProtection {
        LoadBalancerProtection {
            delete,
        }
    }
}


