/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateServerResponse : Response to POST https://api.hetzner.cloud/v1/servers
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServerResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
    #[serde(rename = "next_actions")]
    pub next_actions: Vec<models::Action>,
    /// Root password when no SSH keys have been specified
    #[serde(rename = "root_password", deserialize_with = "Option::deserialize")]
    pub root_password: Option<String>,
    #[serde(rename = "server")]
    pub server: Box<models::Server>,
}

impl CreateServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers
    pub fn new(
        action: models::Action,
        next_actions: Vec<models::Action>,
        root_password: Option<String>,
        server: models::Server,
    ) -> CreateServerResponse {
        CreateServerResponse {
            action: Box::new(action),
            next_actions,
            root_password,
            server: Box::new(server),
        }
    }
}
