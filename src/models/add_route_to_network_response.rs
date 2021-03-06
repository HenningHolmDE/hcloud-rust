/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddRouteToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRouteToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AddRouteToNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route
    pub fn new(action: crate::models::Action) -> AddRouteToNetworkResponse {
        AddRouteToNetworkResponse {
            action: Box::new(action),
        }
    }
}


