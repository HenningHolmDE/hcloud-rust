/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ResourceId : ID of the Resource

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceId {
    /// ID of the Resource | ID of the Server
    #[serde(rename = "id")]
    pub id: i64,
}

impl ResourceId {
    #![allow(clippy::too_many_arguments)]
    /// ID of the Resource
    pub fn new(id: i64) -> ResourceId {
        ResourceId { id }
    }
}
