/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFloatingIpResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::FloatingIp>,
}

impl CreateFloatingIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips
    pub fn new(floating_ip: crate::models::FloatingIp) -> CreateFloatingIpResponse {
        CreateFloatingIpResponse {
            action: None,
            floating_ip: Box::new(floating_ip),
        }
    }
}
