/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceFloatingIpResponse : Response to PUT https://api.hetzner.cloud/v1/floating_ips/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceFloatingIpResponse {
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::FloatingIp>,
}

impl ReplaceFloatingIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/floating_ips/{id}
    pub fn new(floating_ip: crate::models::FloatingIp) -> ReplaceFloatingIpResponse {
        ReplaceFloatingIpResponse {
            floating_ip: Box::new(floating_ip),
        }
    }
}
