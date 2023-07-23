/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForThisServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_dns_ptr

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForThisServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeReverseDnsEntryForThisServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_dns_ptr
    pub fn new(action: crate::models::Action) -> ChangeReverseDnsEntryForThisServerResponse {
        ChangeReverseDnsEntryForThisServerResponse {
            action: Box::new(action),
        }
    }
}
