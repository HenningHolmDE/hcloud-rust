/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RebuildServerFromImageRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RebuildServerFromImageRequest {
    /// ID or name of Image to rebuilt from.
    #[serde(rename = "image")]
    pub image: String,
}

impl RebuildServerFromImageRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild
    pub fn new(image: String) -> RebuildServerFromImageRequest {
        RebuildServerFromImageRequest { image }
    }
}
