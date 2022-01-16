/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCertificateResponse : Response to GET https://api.hetzner.cloud/v1/certificates/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCertificateResponse {
    #[serde(rename = "certificate")]
    pub certificate: Box<crate::models::Certificate>,
}

impl GetCertificateResponse {
    /// Response to GET https://api.hetzner.cloud/v1/certificates/{id}
    pub fn new(certificate: crate::models::Certificate) -> GetCertificateResponse {
        GetCertificateResponse {
            certificate: Box::new(certificate),
        }
    }
}


