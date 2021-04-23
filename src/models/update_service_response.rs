/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateServiceResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/update_service



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateServiceResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl UpdateServiceResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/update_service
    pub fn new(action: crate::models::Action) -> UpdateServiceResponse {
        UpdateServiceResponse {
            action: Box::new(action),
        }
    }
}


