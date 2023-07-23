/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RequestConsoleForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/request_console

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestConsoleForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
    /// VNC password to use for this connection (this password only works in combination with a wss_url with valid token)
    #[serde(rename = "password")]
    pub password: String,
    /// URL of websocket proxy to use; this includes a token which is valid for a limited time only
    #[serde(rename = "wss_url")]
    pub wss_url: String,
}

impl RequestConsoleForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/request_console
    pub fn new(
        action: crate::models::Action,
        password: String,
        wss_url: String,
    ) -> RequestConsoleForServerResponse {
        RequestConsoleForServerResponse {
            action: Box::new(action),
            password,
            wss_url,
        }
    }
}
