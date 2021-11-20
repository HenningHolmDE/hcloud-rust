/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerResponse : Response to POST https://api.hetzner.cloud/v1/servers



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerResponse {
    #[serde(rename = "server")]
    pub server: Box<crate::models::Server>,
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
    #[serde(rename = "next_actions")]
    pub next_actions: Vec<crate::models::Action>,
    /// Root password when no SSH keys have been specified
    #[serde(rename = "root_password")]
    pub root_password: Option<String>,
}

impl CreateServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers
    pub fn new(server: crate::models::Server, action: crate::models::Action, next_actions: Vec<crate::models::Action>, root_password: Option<String>) -> CreateServerResponse {
        CreateServerResponse {
            server: Box::new(server),
            action: Box::new(action),
            next_actions,
            root_password,
        }
    }
}


