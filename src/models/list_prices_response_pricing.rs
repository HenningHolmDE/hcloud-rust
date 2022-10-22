/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricing {
    /// Currency the returned prices are expressed in, coded according to ISO 4217
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::ListPricesResponsePricingFloatingIp>,
    /// Costs of Floating IPs types per Location and type
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<crate::models::ListPricesResponsePricingFloatingIps>,
    #[serde(rename = "image")]
    pub image: Box<crate::models::ListPricesResponsePricingImage>,
    /// Costs of Load Balancer types per Location and type
    #[serde(rename = "load_balancer_types")]
    pub load_balancer_types: Vec<crate::models::ListPricesResponsePricingLoadBalancerTypes>,
    /// Costs of Primary IPs types per Location
    #[serde(rename = "primary_ips")]
    pub primary_ips: Vec<crate::models::ListPricesResponsePricingPrimaryIps>,
    #[serde(rename = "server_backup")]
    pub server_backup: Box<crate::models::ListPricesResponsePricingServerBackup>,
    /// Costs of Server types per Location and type
    #[serde(rename = "server_types")]
    pub server_types: Vec<crate::models::ListPricesResponsePricingServerTypes>,
    #[serde(rename = "traffic")]
    pub traffic: Box<crate::models::ListPricesResponsePricingTraffic>,
    /// The VAT rate used for calculating prices with VAT
    #[serde(rename = "vat_rate")]
    pub vat_rate: String,
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::ListPricesResponsePricingVolume>,
}

impl ListPricesResponsePricing {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        currency: String,
        floating_ip: crate::models::ListPricesResponsePricingFloatingIp,
        floating_ips: Vec<crate::models::ListPricesResponsePricingFloatingIps>,
        image: crate::models::ListPricesResponsePricingImage,
        load_balancer_types: Vec<crate::models::ListPricesResponsePricingLoadBalancerTypes>,
        primary_ips: Vec<crate::models::ListPricesResponsePricingPrimaryIps>,
        server_backup: crate::models::ListPricesResponsePricingServerBackup,
        server_types: Vec<crate::models::ListPricesResponsePricingServerTypes>,
        traffic: crate::models::ListPricesResponsePricingTraffic,
        vat_rate: String,
        volume: crate::models::ListPricesResponsePricingVolume,
    ) -> ListPricesResponsePricing {
        ListPricesResponsePricing {
            currency,
            floating_ip: Box::new(floating_ip),
            floating_ips,
            image: Box::new(image),
            load_balancer_types,
            primary_ips,
            server_backup: Box::new(server_backup),
            server_types,
            traffic: Box::new(traffic),
            vat_rate,
            volume: Box::new(volume),
        }
    }
}
