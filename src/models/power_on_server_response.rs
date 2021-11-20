/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PowerOnServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweron



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PowerOnServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl PowerOnServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweron
    pub fn new(action: crate::models::Action) -> PowerOnServerResponse {
        PowerOnServerResponse {
            action: Box::new(action),
        }
    }
}


