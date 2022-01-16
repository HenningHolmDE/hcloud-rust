/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddTargetRequestServer : Configuration for type Server, required if type is `server`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddTargetRequestServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i32,
}

impl AddTargetRequestServer {
    /// Configuration for type Server, required if type is `server`
    pub fn new(id: i32) -> AddTargetRequestServer {
        AddTargetRequestServer {
            id,
        }
    }
}


