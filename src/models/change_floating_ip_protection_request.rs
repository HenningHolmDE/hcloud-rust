/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeFloatingIpProtectionRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeFloatingIpProtectionRequest {
    /// If true, prevents the Floating IP from being deleted
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}

impl ChangeFloatingIpProtectionRequest {
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection
    pub fn new() -> ChangeFloatingIpProtectionRequest {
        ChangeFloatingIpProtectionRequest {
            delete: None,
        }
    }
}


