/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListDatacentersResponse : Response to GET https://api.hetzner.cloud/v1/datacenters



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDatacentersResponse {
    #[serde(rename = "datacenters")]
    pub datacenters: Vec<crate::models::Datacenter>,
    /// The Datacenter which is recommended to be used to create new Servers.
    #[serde(rename = "recommendation")]
    pub recommendation: i32,
}

impl ListDatacentersResponse {
    /// Response to GET https://api.hetzner.cloud/v1/datacenters
    pub fn new(datacenters: Vec<crate::models::Datacenter>, recommendation: i32) -> ListDatacentersResponse {
        ListDatacentersResponse {
            datacenters,
            recommendation,
        }
    }
}


