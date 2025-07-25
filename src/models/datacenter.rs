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

/// Datacenter : [Datacenter](#datacenters) the [Primary IP](#primary-ips) is located at. | Datacenter this Resource is located at.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datacenter {
    /// Descriptive name for the [Datacenter](#datacenters).  Desired to be easy to understand for humans. Might be changed for cosmetic reasons. Do not use this as an identifier.
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Datacenter.
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "location")]
    pub location: Box<models::Location>,
    /// Unique name for the [Datacenter](#datacenters).  Can be used as a more descriptive identifier.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "server_types")]
    pub server_types: Box<models::DatacenterServerTypes>,
}

impl Datacenter {
    /// [Datacenter](#datacenters) the [Primary IP](#primary-ips) is located at. | Datacenter this Resource is located at.
    pub fn new(
        description: String,
        id: i64,
        location: models::Location,
        name: String,
        server_types: models::DatacenterServerTypes,
    ) -> Datacenter {
        Datacenter {
            description,
            id,
            location: Box::new(location),
            name,
            server_types: Box::new(server_types),
        }
    }
}
