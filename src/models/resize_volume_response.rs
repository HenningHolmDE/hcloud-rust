/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ResizeVolumeResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResizeVolumeResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ResizeVolumeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize
    pub fn new(action: crate::models::Action) -> ResizeVolumeResponse {
        ResizeVolumeResponse {
            action: Box::new(action),
        }
    }
}
