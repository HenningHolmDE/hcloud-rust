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

/// ReplaceNetworkRequest : Request for PUT https://api.hetzner.cloud/v1/networks/{id}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceNetworkRequest {
    /// Toggle to expose routes to the [Networks](#networks) vSwitch.  Indicates if the routes from this [Network](#networks) should be exposed to the vSwitch in this [Network](#networks). Only takes effect if a [vSwitch is setup](https://docs.hetzner.com/cloud/networks/connect-dedi-vswitch) in this [Network](#networks).
    #[serde(
        rename = "expose_routes_to_vswitch",
        skip_serializing_if = "Option::is_none"
    )]
    pub expose_routes_to_vswitch: Option<bool>,
    /// User-defined labels (`key/value` pairs) for the Resource. For more information, see \"[Labels](#labels)\".  | User-defined labels (`key/value` pairs) for the Resource.  Note that the set of [Labels](#labels) provided in the request will overwrite the existing one.  For more information, see \"[Labels](#labels)\".
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// New [Network](#networks) name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceNetworkRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/networks/{id}
    pub fn new() -> ReplaceNetworkRequest {
        ReplaceNetworkRequest {
            expose_routes_to_vswitch: None,
            labels: None,
            name: None,
        }
    }
}
