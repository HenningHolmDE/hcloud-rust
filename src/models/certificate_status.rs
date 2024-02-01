/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CertificateStatus : Current status of a type `managed` Certificate, always *null* for type `uploaded` Certificates

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CertificateStatus {
    #[serde(
        rename = "error",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Option<Box<crate::models::CertificateStatusError>>>,
    /// Status of the issuance process of the Certificate
    #[serde(rename = "issuance", skip_serializing_if = "Option::is_none")]
    pub issuance: Option<Issuance>,
    /// Status of the renewal process of the Certificate.
    #[serde(rename = "renewal", skip_serializing_if = "Option::is_none")]
    pub renewal: Option<Renewal>,
}

impl CertificateStatus {
    #![allow(clippy::too_many_arguments)]
    /// Current status of a type `managed` Certificate, always *null* for type `uploaded` Certificates
    pub fn new() -> CertificateStatus {
        CertificateStatus {
            error: None,
            issuance: None,
            renewal: None,
        }
    }
}

/// Status of the issuance process of the Certificate
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Issuance {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Issuance {
    fn default() -> Issuance {
        Self::Completed
    }
}
/// Status of the renewal process of the Certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Renewal {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl Default for Renewal {
    fn default() -> Renewal {
        Self::Failed
    }
}
