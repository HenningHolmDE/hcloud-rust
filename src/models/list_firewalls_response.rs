/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListFirewallsResponse : Response to GET https://api.hetzner.cloud/v1/firewalls



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFirewallsResponse {
    #[serde(rename = "firewalls")]
    pub firewalls: Vec<crate::models::Firewall>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListFirewallsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/firewalls
    pub fn new(firewalls: Vec<crate::models::Firewall>) -> ListFirewallsResponse {
        ListFirewallsResponse {
            firewalls,
            meta: None,
        }
    }
}


