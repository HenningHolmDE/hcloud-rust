/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GetCertificateResponse : Response to GET https://api.hetzner.cloud/v1/certificates/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCertificateResponse {
    #[serde(rename = "certificate")]
    pub certificate: Box<models::Certificate>,
}

impl GetCertificateResponse {
    /// Response to GET https://api.hetzner.cloud/v1/certificates/{id}
    pub fn new(certificate: models::Certificate) -> GetCertificateResponse {
        GetCertificateResponse {
            certificate: Box::new(certificate),
        }
    }
}
