/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponsePricingServerTypes {
    /// ID of the [Server Types](#server-types) the price is for.
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the [Server Types](#server-types) the price is for.
    #[serde(rename = "name")]
    pub name: String,
    /// Price of the [Server Types](#server-types) per [Location](#locations).
    #[serde(rename = "prices")]
    pub prices: Vec<models::PricePerTime>,
}

impl ListPricesResponsePricingServerTypes {
    pub fn new(
        id: i64,
        name: String,
        prices: Vec<models::PricePerTime>,
    ) -> ListPricesResponsePricingServerTypes {
        ListPricesResponsePricingServerTypes { id, name, prices }
    }
}
