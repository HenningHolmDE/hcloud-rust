/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceVolumeResponse : Response to PUT https://api.hetzner.cloud/v1/volumes/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceVolumeResponse {
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::Volume>,
}

impl ReplaceVolumeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new(volume: crate::models::Volume) -> ReplaceVolumeResponse {
        ReplaceVolumeResponse {
            volume: Box::new(volume),
        }
    }
}
