/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceLoadBalancerRequest : Request for PUT https://api.hetzner.cloud/v1/load_balancers/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceLoadBalancerRequest {
    /// New Load Balancer name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl ReplaceLoadBalancerRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/load_balancers/{id}
    pub fn new() -> ReplaceLoadBalancerRequest {
        ReplaceLoadBalancerRequest {
            name: None,
            labels: None,
        }
    }
}


