/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetDatacenterResponse : Response to GET https://api.hetzner.cloud/v1/datacenters/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDatacenterResponse {
    #[serde(rename = "datacenter")]
    pub datacenter: Box<crate::models::Datacenter>,
}

impl GetDatacenterResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/datacenters/{id}
    pub fn new(datacenter: crate::models::Datacenter) -> GetDatacenterResponse {
        GetDatacenterResponse {
            datacenter: Box::new(datacenter),
        }
    }
}
