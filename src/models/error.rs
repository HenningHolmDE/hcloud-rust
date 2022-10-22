/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Error : Error message for the Action if error occurred, otherwise null

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// Fixed machine readable code
    #[serde(rename = "code")]
    pub code: String,
    /// Humanized error message
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    #![allow(clippy::too_many_arguments)]
    /// Error message for the Action if error occurred, otherwise null
    pub fn new(code: String, message: String) -> Error {
        Error { code, message }
    }
}
