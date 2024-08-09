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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponsePricingFloatingIps {
    /// Floating IP type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<models::PricePerTimeMonthly>,
    #[serde(rename = "type")]
    pub r#type: models::IpType,
}

impl ListPricesResponsePricingFloatingIps {
    pub fn new(
        prices: Vec<models::PricePerTimeMonthly>,
        r#type: models::IpType,
    ) -> ListPricesResponsePricingFloatingIps {
        ListPricesResponsePricingFloatingIps { prices, r#type }
    }
}
