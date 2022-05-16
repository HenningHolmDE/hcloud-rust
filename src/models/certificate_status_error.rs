/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CertificateStatusError : If issuance or renewal reports `failed`, this property contains information about what happened

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CertificateStatusError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CertificateStatusError {
    /// If issuance or renewal reports `failed`, this property contains information about what happened
    pub fn new() -> CertificateStatusError {
        CertificateStatusError {
            code: None,
            message: None,
        }
    }
}
