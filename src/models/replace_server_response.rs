/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceServerResponse : Response to PUT https://api.hetzner.cloud/v1/servers/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceServerResponse {
    #[serde(rename = "server")]
    pub server: crate::models::ReplaceServerResponseServer,
}

impl ReplaceServerResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/servers/{id}
    pub fn new(server: crate::models::ReplaceServerResponseServer) -> ReplaceServerResponse {
        ReplaceServerResponse {
            server,
        }
    }
}


