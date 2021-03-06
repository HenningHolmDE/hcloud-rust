/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnableRescueModeForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_rescue



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRescueModeForServerResponse {
    /// Password that will be set for this Server once the Action succeeds
    #[serde(rename = "root_password", skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl EnableRescueModeForServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_rescue
    pub fn new() -> EnableRescueModeForServerResponse {
        EnableRescueModeForServerResponse {
            root_password: None,
            action: None,
        }
    }
}


