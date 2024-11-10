/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GetMetricsForServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}/metrics
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsForServerResponse {
    #[serde(rename = "metrics")]
    pub metrics: Box<models::Metrics>,
}

impl GetMetricsForServerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}/metrics
    pub fn new(metrics: models::Metrics) -> GetMetricsForServerResponse {
        GetMetricsForServerResponse {
            metrics: Box::new(metrics),
        }
    }
}
