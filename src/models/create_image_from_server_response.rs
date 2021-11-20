/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateImageFromServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/create_image



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateImageFromServerResponse {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::Image>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
}

impl CreateImageFromServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/create_image
    pub fn new() -> CreateImageFromServerResponse {
        CreateImageFromServerResponse {
            image: None,
            action: None,
        }
    }
}


