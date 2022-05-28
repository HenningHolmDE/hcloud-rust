/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.8.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerRequestFirewalls {
    /// ID of the Firewall
    #[serde(rename = "firewall")]
    pub firewall: i32,
}

impl CreateServerRequestFirewalls {
    #![allow(clippy::too_many_arguments)]
    pub fn new(firewall: i32) -> CreateServerRequestFirewalls {
        CreateServerRequestFirewalls { firewall }
    }
}
