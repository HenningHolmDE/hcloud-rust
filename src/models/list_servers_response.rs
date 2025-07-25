/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ListServersResponse : Response to GET https://api.hetzner.cloud/v1/servers
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListServersResponse {
    #[serde(rename = "meta")]
    pub meta: Box<models::Meta>,
    #[serde(rename = "servers")]
    pub servers: Vec<models::Server>,
}

impl ListServersResponse {
    /// Response to GET https://api.hetzner.cloud/v1/servers
    pub fn new(meta: models::Meta, servers: Vec<models::Server>) -> ListServersResponse {
        ListServersResponse {
            meta: Box::new(meta),
            servers,
        }
    }
}
