/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerPrivateNet {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<i32>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl LoadBalancerPrivateNet {
    pub fn new() -> LoadBalancerPrivateNet {
        LoadBalancerPrivateNet {
            network: None,
            ip: None,
        }
    }
}


