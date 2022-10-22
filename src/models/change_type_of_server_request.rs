/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeTypeOfServerRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeTypeOfServerRequest {
    /// ID or name of Server type the Server should migrate to
    #[serde(rename = "server_type")]
    pub server_type: String,
    /// If false, do not upgrade the disk (this allows downgrading the Server type later)
    #[serde(rename = "upgrade_disk")]
    pub upgrade_disk: bool,
}

impl ChangeTypeOfServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_type
    pub fn new(server_type: String, upgrade_disk: bool) -> ChangeTypeOfServerRequest {
        ChangeTypeOfServerRequest {
            server_type,
            upgrade_disk,
        }
    }
}
