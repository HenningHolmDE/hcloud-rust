/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FirewallResource : Resource a Firewall should be applied to.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FirewallResource {
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::ResourceId>>,
    /// Type of the resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl FirewallResource {
    #![allow(clippy::too_many_arguments)]
    /// Resource a Firewall should be applied to.
    pub fn new() -> FirewallResource {
        FirewallResource {
            label_selector: None,
            server: None,
            _type: None,
        }
    }
}

/// Type of the resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "label_selector")]
    LabelSelector,
    #[serde(rename = "server")]
    Server,
}

impl Default for Type {
    fn default() -> Type {
        Self::LabelSelector
    }
}
