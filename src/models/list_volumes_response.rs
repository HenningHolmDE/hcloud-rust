/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListVolumesResponse : Response to GET https://api.hetzner.cloud/v1/volumes

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListVolumesResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
    #[serde(rename = "volumes")]
    pub volumes: Vec<crate::models::Volume>,
}

impl ListVolumesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/volumes
    pub fn new(volumes: Vec<crate::models::Volume>) -> ListVolumesResponse {
        ListVolumesResponse {
            meta: None,
            volumes,
        }
    }
}
