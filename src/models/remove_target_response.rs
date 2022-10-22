/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RemoveTargetResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveTargetResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl RemoveTargetResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
    pub fn new(action: crate::models::Action) -> RemoveTargetResponse {
        RemoveTargetResponse {
            action: Box::new(action),
        }
    }
}
