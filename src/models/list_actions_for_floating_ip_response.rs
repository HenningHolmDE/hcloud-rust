/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForFloatingIpResponse : Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}/actions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListActionsForFloatingIpResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListActionsForFloatingIpResponse {
    /// Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForFloatingIpResponse {
        ListActionsForFloatingIpResponse {
            actions,
            meta: None,
        }
    }
}


