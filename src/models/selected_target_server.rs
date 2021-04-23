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
pub struct SelectedTargetServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i32,
}

impl SelectedTargetServer {
    pub fn new(id: i32) -> SelectedTargetServer {
        SelectedTargetServer {
            id,
        }
    }
}


