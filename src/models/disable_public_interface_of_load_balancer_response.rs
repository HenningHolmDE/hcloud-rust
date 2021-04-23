/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DisablePublicInterfaceOfLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/disable_public_interface



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisablePublicInterfaceOfLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DisablePublicInterfaceOfLoadBalancerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/disable_public_interface
    pub fn new(action: crate::models::Action) -> DisablePublicInterfaceOfLoadBalancerResponse {
        DisablePublicInterfaceOfLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}


