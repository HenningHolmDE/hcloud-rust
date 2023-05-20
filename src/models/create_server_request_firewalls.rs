/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerRequestFirewalls {
    /// ID of the Firewall
    #[serde(rename = "firewall")]
    pub firewall: i64,
}

impl CreateServerRequestFirewalls {
    #![allow(clippy::too_many_arguments)]
    pub fn new(firewall: i64) -> CreateServerRequestFirewalls {
        CreateServerRequestFirewalls { firewall }
    }
}
