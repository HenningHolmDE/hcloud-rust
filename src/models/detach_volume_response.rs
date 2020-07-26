/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DetachVolumeResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/detach



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachVolumeResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DetachVolumeResponse {
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/detach
    pub fn new(action: crate::models::Action) -> DetachVolumeResponse {
        DetachVolumeResponse {
            action,
        }
    }
}


