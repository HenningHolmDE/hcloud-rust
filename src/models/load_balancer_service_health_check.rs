/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerServiceHealthCheck : Service health check

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerServiceHealthCheck {
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<crate::models::LoadBalancerServiceHealthCheckHttp>>,
    /// Time interval in seconds health checks are performed
    #[serde(rename = "interval")]
    pub interval: i32,
    /// Port the health check will be performed on
    #[serde(rename = "port")]
    pub port: i32,
    /// Type of the health check
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// Unsuccessful retries needed until a target is considered unhealthy; an unhealthy target needs the same number of successful retries to become healthy again
    #[serde(rename = "retries")]
    pub retries: i32,
    /// Time in seconds after an attempt is considered a timeout
    #[serde(rename = "timeout")]
    pub timeout: i32,
}

impl LoadBalancerServiceHealthCheck {
    #![allow(clippy::too_many_arguments)]
    /// Service health check
    pub fn new(
        interval: i32,
        port: i32,
        protocol: Protocol,
        retries: i32,
        timeout: i32,
    ) -> LoadBalancerServiceHealthCheck {
        LoadBalancerServiceHealthCheck {
            http: None,
            interval,
            port,
            protocol,
            retries,
            timeout,
        }
    }
}

/// Type of the health check
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "tcp")]
    Tcp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Http
    }
}
