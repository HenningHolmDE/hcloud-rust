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

/// ReplacePrimaryIpResponse : Response to PUT https://api.hetzner.cloud/v1/primary_ips/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacePrimaryIpResponse {
    #[serde(rename = "primary_ip")]
    pub primary_ip: Box<models::PrimaryIp>,
}

impl ReplacePrimaryIpResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/primary_ips/{id}
    pub fn new(primary_ip: models::PrimaryIp) -> ReplacePrimaryIpResponse {
        ReplacePrimaryIpResponse {
            primary_ip: Box::new(primary_ip),
        }
    }
}
