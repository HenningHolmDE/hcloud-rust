/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplacePrimaryIpResponse : Response to PUT https://api.hetzner.cloud/v1/primary_ips/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplacePrimaryIpResponse {
    #[serde(rename = "primary_ip")]
    pub primary_ip: Box<crate::models::PrimaryIp>,
}

impl ReplacePrimaryIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/primary_ips/{id}
    pub fn new(primary_ip: crate::models::PrimaryIp) -> ReplacePrimaryIpResponse {
        ReplacePrimaryIpResponse {
            primary_ip: Box::new(primary_ip),
        }
    }
}
