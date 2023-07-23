/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Meta : Metadata contained in the response

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "pagination")]
    pub pagination: Box<crate::models::Pagination>,
}

impl Meta {
    #![allow(clippy::too_many_arguments)]
    /// Metadata contained in the response
    pub fn new(pagination: crate::models::Pagination) -> Meta {
        Meta {
            pagination: Box::new(pagination),
        }
    }
}
