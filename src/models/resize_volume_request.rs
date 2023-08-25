/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ResizeVolumeRequest : Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResizeVolumeRequest {
    /// New Volume size in GB (must be greater than current size)
    #[serde(rename = "size")]
    pub size: f32,
}

impl ResizeVolumeRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize
    pub fn new(size: f32) -> ResizeVolumeRequest {
        ResizeVolumeRequest { size }
    }
}
