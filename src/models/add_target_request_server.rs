/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AddTargetRequestServer : Configuration for type Server, required if type is `server`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddTargetRequestServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i64,
}

impl AddTargetRequestServer {
    /// Configuration for type Server, required if type is `server`
    pub fn new(id: i64) -> AddTargetRequestServer {
        AddTargetRequestServer { id }
    }
}
