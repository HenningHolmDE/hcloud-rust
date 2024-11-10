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
pub struct LoadBalancer {
    #[serde(rename = "algorithm")]
    pub algorithm: Box<models::LoadBalancerAlgorithm>,
    /// Point in time when the Resource was created (in ISO-8601 format).
    #[serde(rename = "created")]
    pub created: String,
    /// ID of the Load Balancer.
    #[serde(rename = "id")]
    pub id: i64,
    /// Free Traffic for the current billing period in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
    /// Inbound Traffic for the current billing period in bytes
    #[serde(rename = "ingoing_traffic", deserialize_with = "Option::deserialize")]
    pub ingoing_traffic: Option<i64>,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels")]
    pub labels: std::collections::HashMap<String, String>,
    #[serde(rename = "load_balancer_type")]
    pub load_balancer_type: Box<models::LoadBalancerType>,
    #[serde(rename = "location")]
    pub location: Box<models::Location>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Outbound Traffic for the current billing period in bytes
    #[serde(rename = "outgoing_traffic", deserialize_with = "Option::deserialize")]
    pub outgoing_traffic: Option<i64>,
    /// Private networks information
    #[serde(rename = "private_net")]
    pub private_net: Vec<models::LoadBalancerPrivateNet>,
    #[serde(rename = "protection")]
    pub protection: Box<models::Protection>,
    #[serde(rename = "public_net")]
    pub public_net: Box<models::LoadBalancerPublicNet>,
    /// List of services that belong to this Load Balancer
    #[serde(rename = "services")]
    pub services: Vec<models::LoadBalancerService>,
    /// List of targets that belong to this Load Balancer
    #[serde(rename = "targets")]
    pub targets: Vec<models::LoadBalancerTarget>,
}

impl LoadBalancer {
    pub fn new(
        algorithm: models::LoadBalancerAlgorithm,
        created: String,
        id: i64,
        included_traffic: i64,
        ingoing_traffic: Option<i64>,
        labels: std::collections::HashMap<String, String>,
        load_balancer_type: models::LoadBalancerType,
        location: models::Location,
        name: String,
        outgoing_traffic: Option<i64>,
        private_net: Vec<models::LoadBalancerPrivateNet>,
        protection: models::Protection,
        public_net: models::LoadBalancerPublicNet,
        services: Vec<models::LoadBalancerService>,
        targets: Vec<models::LoadBalancerTarget>,
    ) -> LoadBalancer {
        LoadBalancer {
            algorithm: Box::new(algorithm),
            created,
            id,
            included_traffic,
            ingoing_traffic,
            labels,
            load_balancer_type: Box::new(load_balancer_type),
            location: Box::new(location),
            name,
            outgoing_traffic,
            private_net,
            protection: Box::new(protection),
            public_net: Box::new(public_net),
            services,
            targets,
        }
    }
}
