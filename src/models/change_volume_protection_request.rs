/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeVolumeProtectionRequest : Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeVolumeProtectionRequest {
    /// If true, prevents the Volume from being deleted
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}

impl ChangeVolumeProtectionRequest {
    /// Request for POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection
    pub fn new() -> ChangeVolumeProtectionRequest {
        ChangeVolumeProtectionRequest {
            delete: None,
        }
    }
}


