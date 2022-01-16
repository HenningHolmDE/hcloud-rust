/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForCertificateResponse : Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForCertificateResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForCertificateResponse {
    /// Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForCertificateResponse {
        GetActionForCertificateResponse {
            action: Box::new(action),
        }
    }
}


