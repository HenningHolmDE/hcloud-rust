/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteServiceRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteServiceRequest {
    /// The listen port of the service you want to delete
    #[serde(rename = "listen_port")]
    pub listen_port: i32,
}

impl DeleteServiceRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service
    pub fn new(listen_port: i32) -> DeleteServiceRequest {
        DeleteServiceRequest { listen_port }
    }
}
