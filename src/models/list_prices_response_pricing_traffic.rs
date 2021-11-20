/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponsePricingTraffic : The cost of additional traffic per TB



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingTraffic {
    #[serde(rename = "price_per_tb")]
    pub price_per_tb: Box<crate::models::Price>,
}

impl ListPricesResponsePricingTraffic {
    /// The cost of additional traffic per TB
    pub fn new(price_per_tb: crate::models::Price) -> ListPricesResponsePricingTraffic {
        ListPricesResponsePricingTraffic {
            price_per_tb: Box::new(price_per_tb),
        }
    }
}


