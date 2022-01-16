/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeIpRangeOfNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeIpRangeOfNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeIpRangeOfNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
    pub fn new(action: crate::models::Action) -> ChangeIpRangeOfNetworkResponse {
        ChangeIpRangeOfNetworkResponse {
            action: Box::new(action),
        }
    }
}


