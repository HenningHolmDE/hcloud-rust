/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.20.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricePerTime {
    /// Name of the Location the price is for
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "price_hourly")]
    pub price_hourly: Box<models::Price>,
    #[serde(rename = "price_monthly")]
    pub price_monthly: Box<models::Price>,
}

impl PricePerTime {
    pub fn new(
        location: String,
        price_hourly: models::Price,
        price_monthly: models::Price,
    ) -> PricePerTime {
        PricePerTime {
            location,
            price_hourly: Box::new(price_hourly),
            price_monthly: Box::new(price_monthly),
        }
    }
}
