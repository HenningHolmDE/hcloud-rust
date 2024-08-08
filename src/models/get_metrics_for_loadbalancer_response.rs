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

/// GetMetricsForLoadbalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/metrics
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsForLoadbalancerResponse {
    #[serde(rename = "metrics")]
    pub metrics: Box<models::Metrics>,
}

impl GetMetricsForLoadbalancerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/metrics
    pub fn new(metrics: models::Metrics) -> GetMetricsForLoadbalancerResponse {
        GetMetricsForLoadbalancerResponse {
            metrics: Box::new(metrics),
        }
    }
}
