/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListLocationsResponse : Response to GET https://api.hetzner.cloud/v1/locations{?name}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    pub locations: Vec<crate::models::Location>,
}

impl ListLocationsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/locations{?name}
    pub fn new(locations: Vec<crate::models::Location>) -> ListLocationsResponse {
        ListLocationsResponse {
            locations,
        }
    }
}


