/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetLocationResponse : Response to GET https://api.hetzner.cloud/v1/locations/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetLocationResponse {
    #[serde(rename = "location")]
    pub location: Box<crate::models::Location>,
}

impl GetLocationResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/locations/{id}
    pub fn new(location: crate::models::Location) -> GetLocationResponse {
        GetLocationResponse {
            location: Box::new(location),
        }
    }
}
