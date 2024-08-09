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
pub struct PricePerTime {
    /// Free traffic per month in bytes.
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
    /// Name of the Location the price is for. | Name of the Location the price is for
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "price_hourly")]
    pub price_hourly: Box<models::Price>,
    #[serde(rename = "price_monthly")]
    pub price_monthly: Box<models::Price>,
    #[serde(rename = "price_per_tb_traffic")]
    pub price_per_tb_traffic: Box<models::Price>,
}

impl PricePerTime {
    pub fn new(
        included_traffic: i64,
        location: String,
        price_hourly: models::Price,
        price_monthly: models::Price,
        price_per_tb_traffic: models::Price,
    ) -> PricePerTime {
        PricePerTime {
            included_traffic,
            location,
            price_hourly: Box::new(price_hourly),
            price_monthly: Box::new(price_monthly),
            price_per_tb_traffic: Box::new(price_per_tb_traffic),
        }
    }
}
