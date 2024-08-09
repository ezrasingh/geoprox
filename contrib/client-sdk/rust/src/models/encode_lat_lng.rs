/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.1
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EncodeLatLng : Arguments for encoding latitude/longitude as geohash
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodeLatLng {
    /// Determines geohash length
    #[serde(rename = "depth")]
    pub depth: i32,
    /// Latitude
    #[serde(rename = "lat")]
    pub lat: f64,
    /// Longitude
    #[serde(rename = "lng")]
    pub lng: f64,
}

impl EncodeLatLng {
    /// Arguments for encoding latitude/longitude as geohash
    pub fn new(depth: i32, lat: f64, lng: f64) -> EncodeLatLng {
        EncodeLatLng {
            depth,
            lat,
            lng,
        }
    }
}

