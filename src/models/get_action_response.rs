/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetActionResponse : Response to GET https://api.hetzner.cloud/v1/actions/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/actions/{id}
    pub fn new(action: crate::models::Action) -> GetActionResponse {
        GetActionResponse {
            action: Box::new(action),
        }
    }
}
