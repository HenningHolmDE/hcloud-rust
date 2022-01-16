/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Metrics : You must specify the type of metric to get: open_connections, requests_per_second or bandwidth. You can also specify more than one type by comma separation, e.g. requests_per_second,bandwidth. Depending on the type you will get different time series data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// End of period of metrics reported (in ISO-8601 format)
    #[serde(rename = "end")]
    pub end: String,
    /// Start of period of metrics reported (in ISO-8601 format)
    #[serde(rename = "start")]
    pub start: String,
    /// Resolution of results in seconds.
    #[serde(rename = "step")]
    pub step: i32,
    /// Hash with timeseries information, containing the name of timeseries as key
    #[serde(rename = "time_series")]
    pub time_series: ::std::collections::HashMap<String, crate::models::MetricsTimeSeries>,
}

impl Metrics {
    /// You must specify the type of metric to get: open_connections, requests_per_second or bandwidth. You can also specify more than one type by comma separation, e.g. requests_per_second,bandwidth. Depending on the type you will get different time series data.
    pub fn new(end: String, start: String, step: i32, time_series: ::std::collections::HashMap<String, crate::models::MetricsTimeSeries>) -> Metrics {
        Metrics {
            end,
            start,
            step,
            time_series,
        }
    }
}


