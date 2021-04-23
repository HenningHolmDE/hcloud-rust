/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// ID of resource referenced
    #[serde(rename = "id")]
    pub id: i32,
    /// Type of resource referenced
    #[serde(rename = "type")]
    pub _type: String,
}

impl Resource {
    pub fn new(id: i32, _type: String) -> Resource {
        Resource {
            id,
            _type,
        }
    }
}


