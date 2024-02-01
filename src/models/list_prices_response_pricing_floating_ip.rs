/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponsePricingFloatingIp : The cost of one Floating IP per month

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingFloatingIp {
    #[serde(rename = "price_monthly")]
    pub price_monthly: Box<crate::models::Price>,
}

impl ListPricesResponsePricingFloatingIp {
    #![allow(clippy::too_many_arguments)]
    /// The cost of one Floating IP per month
    pub fn new(price_monthly: crate::models::Price) -> ListPricesResponsePricingFloatingIp {
        ListPricesResponsePricingFloatingIp {
            price_monthly: Box::new(price_monthly),
        }
    }
}
