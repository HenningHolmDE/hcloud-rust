/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RebuildServerFromImageResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RebuildServerFromImageResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
    /// New root password when not using SSH keys
    #[serde(rename = "root_password")]
    pub root_password: Option<String>,
}

impl RebuildServerFromImageResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild
    pub fn new(action: crate::models::Action, root_password: Option<String>) -> RebuildServerFromImageResponse {
        RebuildServerFromImageResponse {
            action,
            root_password,
        }
    }
}


