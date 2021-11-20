/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMetricsForLoadbalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/metrics



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMetricsForLoadbalancerResponse {
    #[serde(rename = "metrics")]
    pub metrics: Box<crate::models::Metrics>,
}

impl GetMetricsForLoadbalancerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/metrics
    pub fn new(metrics: crate::models::Metrics) -> GetMetricsForLoadbalancerResponse {
        GetMetricsForLoadbalancerResponse {
            metrics: Box::new(metrics),
        }
    }
}


