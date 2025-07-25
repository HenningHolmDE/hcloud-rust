/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Certificate : TLS/SSL Certificates prove the identity of a Server and are used to encrypt client traffic.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    /// Certificate and chain in PEM format, in order so that each record directly certifies the one preceding.
    #[serde(rename = "certificate", deserialize_with = "Option::deserialize")]
    pub certificate: Option<String>,
    /// Point in time when the Resource was created (in ISO-8601 format).
    #[serde(rename = "created")]
    pub created: String,
    /// Domains and subdomains covered by the Certificate.
    #[serde(rename = "domain_names")]
    pub domain_names: Vec<String>,
    /// SHA256 fingerprint of the Certificate.
    #[serde(rename = "fingerprint", deserialize_with = "Option::deserialize")]
    pub fingerprint: Option<String>,
    /// ID of the Certificate.
    #[serde(rename = "id")]
    pub id: i64,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".  | User-defined labels (`key/value` pairs) for the Resource.  Note that the set of [Labels](#labels) provided in the request will overwrite the existing one.  For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Point in time when the Certificate stops being valid (in ISO-8601 format).
    #[serde(rename = "not_valid_after", deserialize_with = "Option::deserialize")]
    pub not_valid_after: Option<String>,
    /// Point in time when the Certificate becomes valid (in ISO-8601 format).
    #[serde(rename = "not_valid_before", deserialize_with = "Option::deserialize")]
    pub not_valid_before: Option<String>,
    #[serde(
        rename = "status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Option<Box<models::CertificateStatus>>>,
    /// Type of the Certificate.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Resources currently using the Certificate.
    #[serde(rename = "used_by")]
    pub used_by: Vec<models::Resource>,
}

impl Certificate {
    /// TLS/SSL Certificates prove the identity of a Server and are used to encrypt client traffic.
    pub fn new(
        certificate: Option<String>,
        created: String,
        domain_names: Vec<String>,
        fingerprint: Option<String>,
        id: i64,
        labels: std::collections::HashMap<String, String>,
        name: String,
        not_valid_after: Option<String>,
        not_valid_before: Option<String>,
        used_by: Vec<models::Resource>,
    ) -> Certificate {
        Certificate {
            certificate,
            created,
            domain_names,
            fingerprint,
            id,
            labels,
            name,
            not_valid_after,
            not_valid_before,
            status: None,
            r#type: None,
            used_by,
        }
    }
}
/// Type of the Certificate.
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
