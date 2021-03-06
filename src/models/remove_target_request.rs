/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemoveTargetRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveTargetRequest {
    /// Type of the resource
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::AddTargetRequestServer>>,
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<crate::models::AddTargetRequestIp>>,
}

impl RemoveTargetRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/remove_target
    pub fn new(_type: Type) -> RemoveTargetRequest {
        RemoveTargetRequest {
            _type,
            server: None,
            label_selector: None,
            ip: None,
        }
    }
}

/// Type of the resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "label_selector")]
    LabelSelector,
    #[serde(rename = "server")]
    Server,
}

