/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.18.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListServersResponse : Response to GET https://api.hetzner.cloud/v1/servers

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListServersResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
    #[serde(rename = "servers")]
    pub servers: Vec<crate::models::Server>,
}

impl ListServersResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/servers
    pub fn new(servers: Vec<crate::models::Server>) -> ListServersResponse {
        ListServersResponse {
            meta: None,
            servers,
        }
    }
}
