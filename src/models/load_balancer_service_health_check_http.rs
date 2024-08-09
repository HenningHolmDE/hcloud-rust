/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LoadBalancerServiceHealthCheckHttp : Additional configuration for protocol http
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerServiceHealthCheckHttp {
    /// Host header to send in the HTTP request. May not contain spaces, percent or backslash symbols. Can be null, in that case no host header is sent.
    #[serde(rename = "domain", deserialize_with = "Option::deserialize")]
    pub domain: Option<String>,
    /// HTTP path to use for health checks. May not contain literal spaces, use percent-encoding instead.
    #[serde(rename = "path")]
    pub path: String,
    /// String that must be contained in HTTP response in order to pass the health check
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// List of returned HTTP status codes in order to pass the health check. Supports the wildcards `?` for exactly one character and `*` for multiple ones.
    #[serde(rename = "status_codes", skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<Vec<String>>,
    /// Use HTTPS for health check
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
}

impl LoadBalancerServiceHealthCheckHttp {
    /// Additional configuration for protocol http
    pub fn new(domain: Option<String>, path: String) -> LoadBalancerServiceHealthCheckHttp {
        LoadBalancerServiceHealthCheckHttp {
            domain,
            path,
            response: None,
            status_codes: None,
            tls: None,
        }
    }
}
