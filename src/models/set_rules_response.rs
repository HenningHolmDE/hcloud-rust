/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SetRulesResponse : Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetRulesResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl SetRulesResponse {
    /// Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules
    pub fn new(actions: Vec<crate::models::Action>) -> SetRulesResponse {
        SetRulesResponse {
            actions,
            meta: None,
        }
    }
}


