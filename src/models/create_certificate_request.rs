/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateCertificateRequest : Request for POST https://api.hetzner.cloud/v1/certificates

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCertificateRequest {
    /// Certificate and chain in PEM format, in order so that each record directly certifies the one preceding. Required for type `uploaded` Certificates.
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// Domains and subdomains that should be contained in the Certificate issued by *Let's Encrypt*. Required for type `managed` Certificates.
    #[serde(rename = "domain_names", skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Name of the Certificate
    #[serde(rename = "name")]
    pub name: String,
    /// Certificate key in PEM format. Required for type `uploaded` Certificates.
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// Choose between uploading a Certificate in PEM format or requesting a managed *Let's Encrypt* Certificate. If omitted defaults to `uploaded`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl CreateCertificateRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/certificates
    pub fn new(name: String) -> CreateCertificateRequest {
        CreateCertificateRequest {
            certificate: None,
            domain_names: None,
            labels: None,
            name,
            private_key: None,
            _type: None,
        }
    }
}

/// Choose between uploading a Certificate in PEM format or requesting a managed *Let's Encrypt* Certificate. If omitted defaults to `uploaded`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "uploaded")]
    Uploaded,
}

impl Default for Type {
    fn default() -> Type {
        Self::Managed
    }
}
