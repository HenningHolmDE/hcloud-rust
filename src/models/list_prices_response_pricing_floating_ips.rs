/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingFloatingIps {
    /// Floating IP type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::ListPricesResponsePricingPrices>,
    /// The type of the Floating IP
    #[serde(rename = "type")]
    pub _type: Type,
}

impl ListPricesResponsePricingFloatingIps {
    pub fn new(
        prices: Vec<crate::models::ListPricesResponsePricingPrices>,
        _type: Type,
    ) -> ListPricesResponsePricingFloatingIps {
        ListPricesResponsePricingFloatingIps { prices, _type }
    }
}

/// The type of the Floating IP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ipv4
    }
}
