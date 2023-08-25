/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.16.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Http : Configuration option for protocols http and https

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Http {
    /// IDs of the Certificates to use for TLS/SSL termination by the Load Balancer; empty for TLS/SSL passthrough or if `protocol` is \"http\"
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<i64>>,
    /// Lifetime of the cookie used for sticky sessions
    #[serde(rename = "cookie_lifetime", skip_serializing_if = "Option::is_none")]
    pub cookie_lifetime: Option<i32>,
    /// Name of the cookie used for sticky sessions
    #[serde(rename = "cookie_name", skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    /// Redirect HTTP requests to HTTPS. Only available if protocol is \"https\". Default `false`
    #[serde(rename = "redirect_http", skip_serializing_if = "Option::is_none")]
    pub redirect_http: Option<bool>,
    /// Use sticky sessions. Only available if protocol is \"http\" or \"https\". Default `false`
    #[serde(rename = "sticky_sessions", skip_serializing_if = "Option::is_none")]
    pub sticky_sessions: Option<bool>,
}

impl Http {
    #![allow(clippy::too_many_arguments)]
    /// Configuration option for protocols http and https
    pub fn new() -> Http {
        Http {
            certificates: None,
            cookie_lifetime: None,
            cookie_name: None,
            redirect_http: None,
            sticky_sessions: None,
        }
    }
}
