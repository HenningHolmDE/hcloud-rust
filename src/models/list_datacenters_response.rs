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

/// ListDatacentersResponse : Response to GET https://api.hetzner.cloud/v1/datacenters
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListDatacentersResponse {
    /// List of [Datacenters](#datacenters).
    #[serde(rename = "datacenters")]
    pub datacenters: Vec<models::Datacenter>,
    #[serde(rename = "meta")]
    pub meta: Box<models::Meta>,
    /// Recommended [Datacenter](#datacenters) for creating new resources.
    #[serde(rename = "recommendation")]
    pub recommendation: i64,
}

impl ListDatacentersResponse {
    /// Response to GET https://api.hetzner.cloud/v1/datacenters
    pub fn new(
        datacenters: Vec<models::Datacenter>,
        meta: models::Meta,
        recommendation: i64,
    ) -> ListDatacentersResponse {
        ListDatacentersResponse {
            datacenters,
            meta: Box::new(meta),
            recommendation,
        }
    }
}
