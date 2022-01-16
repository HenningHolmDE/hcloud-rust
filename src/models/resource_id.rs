/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResourceId : ID of the Resource



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceId {
    /// ID of the Resource | ID of the Server
    #[serde(rename = "id")]
    pub id: i32,
}

impl ResourceId {
    /// ID of the Resource
    pub fn new(id: i32) -> ResourceId {
        ResourceId {
            id,
        }
    }
}


