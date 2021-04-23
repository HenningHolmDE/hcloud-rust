/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageProtection : Protection configuration for the Image



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProtection {
    /// If true, prevents the snapshot from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl ImageProtection {
    /// Protection configuration for the Image
    pub fn new(delete: bool) -> ImageProtection {
        ImageProtection {
            delete,
        }
    }
}


