/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DisableBackupsForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableBackupsForServerResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DisableBackupsForServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup
    pub fn new(action: crate::models::Action) -> DisableBackupsForServerResponse {
        DisableBackupsForServerResponse {
            action,
        }
    }
}


