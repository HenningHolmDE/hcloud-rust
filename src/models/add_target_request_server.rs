/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddTargetRequestServer : Configuration for type Server, required if type is `server`

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddTargetRequestServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i64,
}

impl AddTargetRequestServer {
    #![allow(clippy::too_many_arguments)]
    /// Configuration for type Server, required if type is `server`
    pub fn new(id: i64) -> AddTargetRequestServer {
        AddTargetRequestServer { id }
    }
}
