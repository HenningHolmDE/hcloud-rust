/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SoftRebootServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reboot

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SoftRebootServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl SoftRebootServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reboot
    pub fn new(action: crate::models::Action) -> SoftRebootServerResponse {
        SoftRebootServerResponse {
            action: Box::new(action),
        }
    }
}
