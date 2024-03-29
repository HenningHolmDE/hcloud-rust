/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Architecture : Type of cpu architecture this image is compatible with. | Type of cpu architecture

/// Type of cpu architecture this image is compatible with. | Type of cpu architecture
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Architecture {
    #[serde(rename = "arm")]
    Arm,
    #[serde(rename = "x86")]
    X86,
}

impl ToString for Architecture {
    fn to_string(&self) -> String {
        match self {
            Self::Arm => String::from("arm"),
            Self::X86 => String::from("x86"),
        }
    }
}

impl Default for Architecture {
    fn default() -> Architecture {
        Self::Arm
    }
}
