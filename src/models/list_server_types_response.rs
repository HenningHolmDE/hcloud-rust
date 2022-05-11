/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListServerTypesResponse : Response to GET https://api.hetzner.cloud/v1/server_types

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListServerTypesResponse {
    #[serde(rename = "server_types")]
    pub server_types: Vec<crate::models::ServerType>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListServerTypesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/server_types
    pub fn new(server_types: Vec<crate::models::ServerType>) -> ListServerTypesResponse {
        ListServerTypesResponse {
            server_types,
            meta: None,
        }
    }
}
