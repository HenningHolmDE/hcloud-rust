/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeImageProtectionResponse : Response to POST https://api.hetzner.cloud/v1/images/{id}/actions/change_protection

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeImageProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeImageProtectionResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/images/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeImageProtectionResponse {
        ChangeImageProtectionResponse {
            action: Box::new(action),
        }
    }
}
