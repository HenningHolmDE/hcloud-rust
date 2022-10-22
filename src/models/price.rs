/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Price : Hourly costs for a Resource in this Location | Monthly costs for a Resource in this Location | Monthly costs for a Floating IP type in this Location | Hourly costs for a Load Balancer type in this network zone | Monthly costs for a Load Balancer type in this network zone | Hourly costs for a Primary IP type in this Location | Monthly costs for a Primary IP type in this Location | Hourly costs for a Server type in this Location | Monthly costs for a Server type in this Location

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Price {
    /// Price with VAT added
    #[serde(rename = "gross")]
    pub gross: String,
    /// Price without VAT
    #[serde(rename = "net")]
    pub net: String,
}

impl Price {
    #![allow(clippy::too_many_arguments)]
    /// Hourly costs for a Resource in this Location | Monthly costs for a Resource in this Location | Monthly costs for a Floating IP type in this Location | Hourly costs for a Load Balancer type in this network zone | Monthly costs for a Load Balancer type in this network zone | Hourly costs for a Primary IP type in this Location | Monthly costs for a Primary IP type in this Location | Hourly costs for a Server type in this Location | Monthly costs for a Server type in this Location
    pub fn new(gross: String, net: String) -> Price {
        Price { gross, net }
    }
}
