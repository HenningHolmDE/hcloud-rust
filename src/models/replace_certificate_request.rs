/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.17.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceCertificateRequest : Request for PUT https://api.hetzner.cloud/v1/certificates/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceCertificateRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New Certificate name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceCertificateRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for PUT https://api.hetzner.cloud/v1/certificates/{id}
    pub fn new() -> ReplaceCertificateRequest {
        ReplaceCertificateRequest {
            labels: None,
            name: None,
        }
    }
}
