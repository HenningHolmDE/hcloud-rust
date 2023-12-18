/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListLoadBalancersResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListLoadBalancersResponse {
    #[serde(rename = "load_balancers")]
    pub load_balancers: Vec<crate::models::LoadBalancer>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListLoadBalancersResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers
    pub fn new(load_balancers: Vec<crate::models::LoadBalancer>) -> ListLoadBalancersResponse {
        ListLoadBalancersResponse {
            load_balancers,
            meta: None,
        }
    }
}
