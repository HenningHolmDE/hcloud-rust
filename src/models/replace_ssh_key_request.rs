/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceSshKeyRequest : Request for PUT https://api.hetzner.cloud/v1/ssh_keys/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSshKeyRequest {
    /// New name Name to set
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl ReplaceSshKeyRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/ssh_keys/{id}
    pub fn new() -> ReplaceSshKeyRequest {
        ReplaceSshKeyRequest {
            name: None,
            labels: None,
        }
    }
}


