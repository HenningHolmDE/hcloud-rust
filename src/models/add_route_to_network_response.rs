/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AddRouteToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRouteToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<models::Action>,
}

impl AddRouteToNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route
    pub fn new(action: models::Action) -> AddRouteToNetworkResponse {
        AddRouteToNetworkResponse {
            action: Box::new(action),
        }
    }
}
