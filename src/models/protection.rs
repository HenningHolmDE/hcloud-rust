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

/// Protection : Protection configuration for the Resource
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Protection {
    /// Prevent the Resource from being deleted.
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl Protection {
    /// Protection configuration for the Resource
    pub fn new(delete: bool) -> Protection {
        Protection { delete }
    }
}
