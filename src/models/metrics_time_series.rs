/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsTimeSeries {
    /// Metrics Timestamps with values
    #[serde(rename = "values")]
    pub values: Vec<Vec<serde_json::Value>>,
}

impl MetricsTimeSeries {
    pub fn new(values: Vec<Vec<serde_json::Value>>) -> MetricsTimeSeries {
        MetricsTimeSeries {
            values,
        }
    }
}


