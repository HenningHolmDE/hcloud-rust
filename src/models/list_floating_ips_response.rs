/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListFloatingIpsResponse : Response to GET https://api.hetzner.cloud/v1/floating_ips

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFloatingIpsResponse {
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<crate::models::FloatingIp>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListFloatingIpsResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/floating_ips
    pub fn new(floating_ips: Vec<crate::models::FloatingIp>) -> ListFloatingIpsResponse {
        ListFloatingIpsResponse {
            floating_ips,
            meta: None,
        }
    }
}
