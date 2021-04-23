/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateStatus : Current status of a type `managed` Certificate, always *null* for type `uploaded` Certificates



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateStatus {
    /// Status of the issuance process of the Certificate
    #[serde(rename = "issuance", skip_serializing_if = "Option::is_none")]
    pub issuance: Option<Issuance>,
    /// Status of the renewal process of the Certificate.
    #[serde(rename = "renewal", skip_serializing_if = "Option::is_none")]
    pub renewal: Option<Renewal>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::CertificateStatusError>>,
}

impl CertificateStatus {
    /// Current status of a type `managed` Certificate, always *null* for type `uploaded` Certificates
    pub fn new() -> CertificateStatus {
        CertificateStatus {
            issuance: None,
            renewal: None,
            error: None,
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

