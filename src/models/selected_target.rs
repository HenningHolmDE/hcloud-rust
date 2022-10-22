/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelectedTarget {
    #[serde(rename = "health_status", skip_serializing_if = "Option::is_none")]
    pub health_status: Option<Vec<crate::models::HealthStatus>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::ResourceId>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "use_private_ip", skip_serializing_if = "Option::is_none")]
    pub use_private_ip: Option<bool>,
}

impl SelectedTarget {
    #![allow(clippy::too_many_arguments)]
    pub fn new() -> SelectedTarget {
        SelectedTarget {
            health_status: None,
            server: None,
            _type: None,
            use_private_ip: None,
        }
    }
}
