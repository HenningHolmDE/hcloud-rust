/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Datacenter : Datacenter this Resource is located at

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Datacenter {
    /// Description of the Datacenter
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "location")]
    pub location: Box<crate::models::Location>,
    /// Unique identifier of the Datacenter
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "server_types")]
    pub server_types: Box<crate::models::DatacenterServerTypes>,
}

impl Datacenter {
    #![allow(clippy::too_many_arguments)]
    /// Datacenter this Resource is located at
    pub fn new(
        description: String,
        id: i32,
        location: crate::models::Location,
        name: String,
        server_types: crate::models::DatacenterServerTypes,
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
