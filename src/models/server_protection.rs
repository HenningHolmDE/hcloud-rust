/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServerProtection : Protection configuration for the Server

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerProtection {
    /// If true, prevents the Server from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
    /// If true, prevents the Server from being rebuilt
    #[serde(rename = "rebuild")]
    pub rebuild: bool,
}

impl ServerProtection {
    #![allow(clippy::too_many_arguments)]
    /// Protection configuration for the Server
    pub fn new(delete: bool, rebuild: bool) -> ServerProtection {
        ServerProtection { delete, rebuild }
    }
}
