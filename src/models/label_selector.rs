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

/// LabelSelector : Configuration for type LabelSelector, required if type is `label_selector`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelSelector {
    /// Label selector | The selector.
    #[serde(rename = "selector")]
    pub selector: String,
}

impl LabelSelector {
    /// Configuration for type LabelSelector, required if type is `label_selector`
    pub fn new(selector: String) -> LabelSelector {
        LabelSelector { selector }
    }
}
