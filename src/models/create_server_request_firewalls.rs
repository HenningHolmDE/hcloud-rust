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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServerRequestFirewalls {
    /// ID of the Firewall
    #[serde(rename = "firewall")]
    pub firewall: i64,
}

impl CreateServerRequestFirewalls {
    pub fn new(firewall: i64) -> CreateServerRequestFirewalls {
        CreateServerRequestFirewalls { firewall }
    }
}
