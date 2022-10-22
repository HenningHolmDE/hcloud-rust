/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resource {
    /// ID of the Resource | ID of resource referenced
    #[serde(rename = "id")]
    pub id: i32,
    /// Type of resource referenced
    #[serde(rename = "type")]
    pub _type: String,
}

impl Resource {
    #![allow(clippy::too_many_arguments)]
    pub fn new(id: i32, _type: String) -> Resource {
        Resource { id, _type }
    }
}
