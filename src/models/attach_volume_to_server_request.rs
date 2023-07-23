/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachVolumeToServerRequest : Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/attach

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachVolumeToServerRequest {
    /// Auto-mount the Volume after attaching it
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
    /// ID of the Server the Volume will be attached to
    #[serde(rename = "server")]
    pub server: i64,
}

impl AttachVolumeToServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/attach
    pub fn new(server: i64) -> AttachVolumeToServerRequest {
        AttachVolumeToServerRequest {
            automount: None,
            server,
        }
    }
}
