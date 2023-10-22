/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.18.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponsePricingServerBackup : Will increase base Server costs by specific percentage

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingServerBackup {
    /// Percentage by how much the base price will increase
    #[serde(rename = "percentage")]
    pub percentage: String,
}

impl ListPricesResponsePricingServerBackup {
    #![allow(clippy::too_many_arguments)]
    /// Will increase base Server costs by specific percentage
    pub fn new(percentage: String) -> ListPricesResponsePricingServerBackup {
        ListPricesResponsePricingServerBackup { percentage }
    }
}
