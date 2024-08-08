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
pub struct ListPricesResponsePricingLoadBalancerTypes {
    /// ID of the Load Balancer type the price is for
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the Load Balancer type the price is for
    #[serde(rename = "name")]
    pub name: String,
    /// Load Balancer type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<models::PricePerTime>,
}

impl ListPricesResponsePricingLoadBalancerTypes {
    pub fn new(
        id: i64,
        name: String,
        prices: Vec<models::PricePerTime>,
    ) -> ListPricesResponsePricingLoadBalancerTypes {
        ListPricesResponsePricingLoadBalancerTypes { id, name, prices }
    }
}
