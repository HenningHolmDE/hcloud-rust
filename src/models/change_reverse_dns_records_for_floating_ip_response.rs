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

/// ChangeReverseDnsRecordsForFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeReverseDnsRecordsForFloatingIpResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl ChangeReverseDnsRecordsForFloatingIpResponse {
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr
    pub fn new(action: models::Action) -> ChangeReverseDnsRecordsForFloatingIpResponse {
        ChangeReverseDnsRecordsForFloatingIpResponse {
            action: Box::new(action),
        }
    }
}
