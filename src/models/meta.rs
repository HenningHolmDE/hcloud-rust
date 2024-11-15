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
pub struct Meta {
    #[serde(rename = "pagination")]
    pub pagination: Box<models::Pagination>,
}

impl Meta {
    pub fn new(pagination: models::Pagination) -> Meta {
        Meta {
            pagination: Box::new(pagination),
        }
    }
}
