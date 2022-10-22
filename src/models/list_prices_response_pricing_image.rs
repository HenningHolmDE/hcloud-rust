/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponsePricingImage : The cost of Image per GB/month

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingImage {
    #[serde(rename = "price_per_gb_month")]
    pub price_per_gb_month: Box<crate::models::Price>,
}

impl ListPricesResponsePricingImage {
    #![allow(clippy::too_many_arguments)]
    /// The cost of Image per GB/month
    pub fn new(price_per_gb_month: crate::models::Price) -> ListPricesResponsePricingImage {
        ListPricesResponsePricingImage {
            price_per_gb_month: Box::new(price_per_gb_month),
        }
    }
}
