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

/// ListPrimaryIpsResponse : Response to GET https://api.hetzner.cloud/v1/primary_ips
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPrimaryIpsResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::Meta>>,
    #[serde(rename = "primary_ips")]
    pub primary_ips: Vec<models::PrimaryIp>,
}

impl ListPrimaryIpsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/primary_ips
    pub fn new(primary_ips: Vec<models::PrimaryIp>) -> ListPrimaryIpsResponse {
        ListPrimaryIpsResponse {
            meta: None,
            primary_ips,
        }
    }
}
