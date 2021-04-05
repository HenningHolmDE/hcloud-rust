/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForCertificateResponse : Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListActionsForCertificateResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::models::Meta>,
}

impl ListActionsForCertificateResponse {
    /// Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForCertificateResponse {
        ListActionsForCertificateResponse {
            actions,
            meta: None,
        }
    }
}

