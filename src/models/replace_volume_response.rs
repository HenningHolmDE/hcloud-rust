/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceVolumeResponse : Response to PUT https://api.hetzner.cloud/v1/volumes/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceVolumeResponse {
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::Volume>,
}

impl ReplaceVolumeResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new(volume: crate::models::Volume) -> ReplaceVolumeResponse {
        ReplaceVolumeResponse {
            volume: Box::new(volume),
        }
    }
}


