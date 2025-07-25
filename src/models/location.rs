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

/// Location : [Location](#locations) the [Datacenter](#datacenters) is located at.  | Location of the Volume. Volume can only be attached to Servers in the same Location.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Name of the closest city to the [Location](#locations).  City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
    #[serde(rename = "city")]
    pub city: String,
    /// Country the [Location](#locations) resides in.  ISO 3166-1 alpha-2 code of the country.
    #[serde(rename = "country")]
    pub country: String,
    /// Human readable description of the [Location](#locations).
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Location.
    #[serde(rename = "id")]
    pub id: i64,
    /// Latitude of the city closest to the [Location](#locations).
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude of the city closest to the [Location](#locations).
    #[serde(rename = "longitude")]
    pub longitude: f64,
    /// Unique identifier of the [Location](#locations).
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the Network Zone this [Location](#locations) resides in.
    #[serde(rename = "network_zone")]
    pub network_zone: String,
}

impl Location {
    /// [Location](#locations) the [Datacenter](#datacenters) is located at.  | Location of the Volume. Volume can only be attached to Servers in the same Location.
    pub fn new(
        city: String,
        country: String,
        description: String,
        id: i64,
        latitude: f64,
        longitude: f64,
        name: String,
        network_zone: String,
    ) -> Location {
        Location {
            city,
            country,
            description,
            id,
            latitude,
            longitude,
            name,
            network_zone,
        }
    }
}
