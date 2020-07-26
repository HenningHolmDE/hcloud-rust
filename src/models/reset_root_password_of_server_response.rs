/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResetRootPasswordOfServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset_password



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetRootPasswordOfServerResponse {
    /// Password that will be set for this Server once the Action succeeds
    #[serde(rename = "root_password", skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl ResetRootPasswordOfServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset_password
    pub fn new(action: crate::models::Action) -> ResetRootPasswordOfServerResponse {
        ResetRootPasswordOfServerResponse {
            root_password: None,
            action,
        }
    }
}


