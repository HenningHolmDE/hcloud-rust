/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ListImagesResponse : Response to GET https://api.hetzner.cloud/v1/images
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListImagesResponse {
    #[serde(rename = "images")]
    pub images: Vec<models::Image>,
    #[serde(rename = "meta")]
    pub meta: Box<models::Meta>,
}

impl ListImagesResponse {
    /// Response to GET https://api.hetzner.cloud/v1/images
    pub fn new(images: Vec<models::Image>, meta: models::Meta) -> ListImagesResponse {
        ListImagesResponse {
            images,
            meta: Box::new(meta),
        }
    }
}
