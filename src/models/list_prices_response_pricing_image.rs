/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ListPricesResponsePricingImage : The cost of Image per GB/month
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponsePricingImage {
    #[serde(rename = "price_per_gb_month")]
    pub price_per_gb_month: Box<models::Price>,
}

impl ListPricesResponsePricingImage {
    /// The cost of Image per GB/month
    pub fn new(price_per_gb_month: models::Price) -> ListPricesResponsePricingImage {
        ListPricesResponsePricingImage {
            price_per_gb_month: Box::new(price_per_gb_month),
        }
    }
}
