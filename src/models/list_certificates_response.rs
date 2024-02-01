/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListCertificatesResponse : Response to GET https://api.hetzner.cloud/v1/certificates

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListCertificatesResponse {
    #[serde(rename = "certificates")]
    pub certificates: Vec<crate::models::Certificate>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListCertificatesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/certificates
    pub fn new(certificates: Vec<crate::models::Certificate>) -> ListCertificatesResponse {
        ListCertificatesResponse {
            certificates,
            meta: None,
        }
    }
}
