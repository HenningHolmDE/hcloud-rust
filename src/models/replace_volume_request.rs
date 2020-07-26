/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceVolumeRequest : Request for PUT https://api.hetzner.cloud/v1/volumes/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceVolumeRequest {
    /// New Volume name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::models::ReplaceFloatingIpRequestLabels>,
}

impl ReplaceVolumeRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new(name: String) -> ReplaceVolumeRequest {
        ReplaceVolumeRequest {
            name,
            labels: None,
        }
    }
}


