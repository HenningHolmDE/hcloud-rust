/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceSshKeyResponse : Response to PUT https://api.hetzner.cloud/v1/ssh_keys/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceSshKeyResponse {
    #[serde(rename = "ssh_key")]
    pub ssh_key: Box<crate::models::SshKey>,
}

impl ReplaceSshKeyResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/ssh_keys/{id}
    pub fn new(ssh_key: crate::models::SshKey) -> ReplaceSshKeyResponse {
        ReplaceSshKeyResponse {
            ssh_key: Box::new(ssh_key),
        }
    }
}


