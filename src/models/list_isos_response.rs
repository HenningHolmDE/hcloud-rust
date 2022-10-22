/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListIsosResponse : Response to GET https://api.hetzner.cloud/v1/isos

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListIsosResponse {
    #[serde(rename = "isos")]
    pub isos: Vec<crate::models::Iso>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListIsosResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/isos
    pub fn new(isos: Vec<crate::models::Iso>) -> ListIsosResponse {
        ListIsosResponse { isos, meta: None }
    }
}
