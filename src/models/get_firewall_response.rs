/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFirewallResponse : Response to GET https://api.hetzner.cloud/v1/firewalls/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFirewallResponse {
    #[serde(rename = "firewall")]
    pub firewall: Box<crate::models::Firewall>,
}

impl GetFirewallResponse {
    /// Response to GET https://api.hetzner.cloud/v1/firewalls/{id}
    pub fn new(firewall: crate::models::Firewall) -> GetFirewallResponse {
        GetFirewallResponse {
            firewall: Box::new(firewall),
        }
    }
}


