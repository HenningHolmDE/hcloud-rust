/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMetricsForServerResponseMetrics {
    /// Start of period of metrics reported (in ISO-8601 format)
    #[serde(rename = "start")]
    pub start: String,
    /// End of period of metrics reported (in ISO-8601 format)
    #[serde(rename = "end")]
    pub end: String,
    /// Resolution of results in seconds.
    #[serde(rename = "step")]
    pub step: i32,
    #[serde(rename = "time_series")]
    pub time_series: ::std::collections::HashMap<String, serde_json::Value>,
}

impl GetMetricsForServerResponseMetrics {
    pub fn new(start: String, end: String, step: i32, time_series: ::std::collections::HashMap<String, serde_json::Value>) -> GetMetricsForServerResponseMetrics {
        GetMetricsForServerResponseMetrics {
            start,
            end,
            step,
            time_series,
        }
    }
}


