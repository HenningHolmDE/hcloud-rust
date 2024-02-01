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
pub struct ListPricesResponsePricingPrimaryIps {
    /// Primary IP type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTime>,
    #[serde(rename = "type")]
    pub r#type: crate::models::IpType,
}

impl ListPricesResponsePricingPrimaryIps {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        prices: Vec<crate::models::PricePerTime>,
        r#type: crate::models::IpType,
    ) -> ListPricesResponsePricingPrimaryIps {
        ListPricesResponsePricingPrimaryIps { prices, r#type }
    }
}
