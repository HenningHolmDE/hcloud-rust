/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FirewallResourceServer : Configuration for type Server, required if type is `server` | Configuration for type server, required if type is `server`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallResourceServer {
    /// ID of the Server
    #[serde(rename = "id")]
    pub id: i32,
}

impl FirewallResourceServer {
    /// Configuration for type Server, required if type is `server` | Configuration for type server, required if type is `server`
    pub fn new(id: i32) -> FirewallResourceServer {
        FirewallResourceServer {
            id,
        }
    }
}


