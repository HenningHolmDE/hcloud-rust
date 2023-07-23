/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PricePerTime {
    /// Name of the Location the price is for
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "price_hourly")]
    pub price_hourly: Box<crate::models::Price>,
    #[serde(rename = "price_monthly")]
    pub price_monthly: Box<crate::models::Price>,
}

impl PricePerTime {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        location: String,
        price_hourly: crate::models::Price,
        price_monthly: crate::models::Price,
    ) -> PricePerTime {
        PricePerTime {
            location,
            price_hourly: Box::new(price_hourly),
            price_monthly: Box::new(price_monthly),
        }
    }
}
