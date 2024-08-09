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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricsTimeSeriesValue {
    Number(f64),
    String(String),
}

impl Default for MetricsTimeSeriesValue {
    fn default() -> Self {
        Self::Number(Default::default())
    }
}
