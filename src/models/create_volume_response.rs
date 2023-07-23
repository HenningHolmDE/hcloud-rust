/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeResponse : Response to POST https://api.hetzner.cloud/v1/volumes

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateVolumeResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
    #[serde(rename = "next_actions")]
    pub next_actions: Vec<crate::models::Action>,
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::Volume>,
}

impl CreateVolumeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/volumes
    pub fn new(
        action: crate::models::Action,
        next_actions: Vec<crate::models::Action>,
        volume: crate::models::Volume,
    ) -> CreateVolumeResponse {
        CreateVolumeResponse {
            action: Box::new(action),
            next_actions,
            volume: Box::new(volume),
        }
    }
}
