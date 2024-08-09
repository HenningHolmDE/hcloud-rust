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

/// SshKey : SSH keys are public keys you provide to the cloud system. They can be injected into Servers at creation time. We highly recommend that you use keys instead of passwords to manage your Servers.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshKey {
    /// Point in time when the Resource was created (in ISO-8601 format).
    #[serde(rename = "created")]
    pub created: String,
    /// Fingerprint of public key
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// ID of the SSH Key.
    #[serde(rename = "id")]
    pub id: i64,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Public key
    #[serde(rename = "public_key")]
    pub public_key: String,
}

impl SshKey {
    /// SSH keys are public keys you provide to the cloud system. They can be injected into Servers at creation time. We highly recommend that you use keys instead of passwords to manage your Servers.
    pub fn new(
        created: String,
        fingerprint: String,
        id: i64,
        labels: std::collections::HashMap<String, String>,
        name: String,
        public_key: String,
    ) -> SshKey {
        SshKey {
            created,
            fingerprint,
            id,
            labels,
            name,
            public_key,
        }
    }
}
