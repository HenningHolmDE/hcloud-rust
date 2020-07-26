/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DatacenterServerTypes : The Server types the Datacenter can handle



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatacenterServerTypes {
    /// IDs of Server types that are supported in the Datacenter
    #[serde(rename = "supported")]
    pub supported: Vec<i32>,
    /// IDs of Server types that are supported and for which the Datacenter has enough resources left
    #[serde(rename = "available")]
    pub available: Vec<i32>,
    /// IDs of Server types that are supported and for which the Datacenter has enough resources left
    #[serde(rename = "available_for_migration")]
    pub available_for_migration: Vec<i32>,
}

impl DatacenterServerTypes {
    /// The Server types the Datacenter can handle
    pub fn new(supported: Vec<i32>, available: Vec<i32>, available_for_migration: Vec<i32>) -> DatacenterServerTypes {
        DatacenterServerTypes {
            supported,
            available,
            available_for_migration,
        }
    }
}


