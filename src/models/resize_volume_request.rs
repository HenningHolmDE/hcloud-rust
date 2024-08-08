/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ResizeVolumeRequest : Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeVolumeRequest {
    /// New Volume size in GB (must be greater than current size)
    #[serde(rename = "size")]
    pub size: f64,
}

impl ResizeVolumeRequest {
    /// Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/resize
    pub fn new(size: f64) -> ResizeVolumeRequest {
        ResizeVolumeRequest { size }
    }
}
