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

/// GetSshKeyResponse : Response to GET https://api.hetzner.cloud/v1/ssh_keys/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSshKeyResponse {
    #[serde(rename = "ssh_key")]
    pub ssh_key: Box<models::SshKey>,
}

impl GetSshKeyResponse {
    /// Response to GET https://api.hetzner.cloud/v1/ssh_keys/{id}
    pub fn new(ssh_key: models::SshKey) -> GetSshKeyResponse {
        GetSshKeyResponse {
            ssh_key: Box::new(ssh_key),
        }
    }
}
