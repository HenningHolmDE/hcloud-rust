/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Iso {
    #[serde(rename = "architecture", deserialize_with = "Option::deserialize")]
    pub architecture: Option<crate::models::Architecture>,
    #[serde(rename = "deprecation", deserialize_with = "Option::deserialize")]
    pub deprecation: Option<Box<crate::models::DeprecationInfo>>,
    /// Description of the ISO
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i64,
    /// Unique identifier of the ISO. Only set for public ISOs
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Type of the ISO
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<Type>,
}

impl Iso {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        architecture: Option<crate::models::Architecture>,
        deprecation: Option<crate::models::DeprecationInfo>,
        description: String,
        id: i64,
        name: Option<String>,
        r#type: Option<Type>,
    ) -> Iso {
        Iso {
            architecture,
            deprecation: deprecation.map(Box::new),
            description,
            id,
            name,
            r#type,
        }
    }
}

/// Type of the ISO
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

impl Default for Type {
    fn default() -> Type {
        Self::Private
    }
}
