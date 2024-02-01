/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetVolumeResponse : Response to GET https://api.hetzner.cloud/v1/volumes/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetVolumeResponse {
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::Volume>,
}

impl GetVolumeResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new(volume: crate::models::Volume) -> GetVolumeResponse {
        GetVolumeResponse {
            volume: Box::new(volume),
        }
    }
}
