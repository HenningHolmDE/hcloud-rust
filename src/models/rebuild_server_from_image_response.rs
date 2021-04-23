/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RebuildServerFromImageResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RebuildServerFromImageResponse {
    /// New root password when not using SSH keys
    #[serde(rename = "root_password", skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl RebuildServerFromImageResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild
    pub fn new() -> RebuildServerFromImageResponse {
        RebuildServerFromImageResponse {
            root_password: None,
            action: None,
        }
    }
}


