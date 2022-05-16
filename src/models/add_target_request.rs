/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddTargetRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/add_target

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddTargetRequest {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<crate::models::AddTargetRequestIp>>,
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::AddTargetRequestServer>>,
    /// Type of the resource
    #[serde(rename = "type")]
    pub _type: Type,
    /// Use the private network IP instead of the public IP of the Server, requires the Server and Load Balancer to be in the same network. Default value is false.
    #[serde(rename = "use_private_ip", skip_serializing_if = "Option::is_none")]
    pub use_private_ip: Option<bool>,
}

impl AddTargetRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/add_target
    pub fn new(_type: Type) -> AddTargetRequest {
        AddTargetRequest {
            ip: None,
            label_selector: None,
            server: None,
            _type,
            use_private_ip: None,
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

impl Default for Type {
    fn default() -> Type {
        Self::Ip
    }
}
