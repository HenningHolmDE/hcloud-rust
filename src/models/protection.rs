/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Protection : Protection configuration for the Resource

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Protection {
    /// If true, prevents the Resource from being deleted | If true, prevents the Network from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl Protection {
    #![allow(clippy::too_many_arguments)]
    /// Protection configuration for the Resource
    pub fn new(delete: bool) -> Protection {
        Protection { delete }
    }
}
