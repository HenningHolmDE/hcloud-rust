/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachVolumeToServerResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/attach

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachVolumeToServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AttachVolumeToServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/attach
    pub fn new(action: crate::models::Action) -> AttachVolumeToServerResponse {
        AttachVolumeToServerResponse {
            action: Box::new(action),
        }
    }
}
