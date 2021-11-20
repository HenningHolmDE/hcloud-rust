/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResetServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ResetServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset
    pub fn new(action: crate::models::Action) -> ResetServerResponse {
        ResetServerResponse {
            action: Box::new(action),
        }
    }
}


