/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.3.1
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DecodeGeohashResponse : Returns geohash decoded as latitude/longitude with precision errors
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodeGeohashResponse {
    /// Latitude
    #[serde(rename = "lat")]
    pub lat: f64,
    /// Latitude error
    #[serde(rename = "lat_error")]
    pub lat_error: f64,
    /// Longitude
    #[serde(rename = "lng")]
    pub lng: f64,
    /// Longitude error
    #[serde(rename = "lng_error")]
    pub lng_error: f64,
}

impl DecodeGeohashResponse {
    /// Returns geohash decoded as latitude/longitude with precision errors
    pub fn new(lat: f64, lat_error: f64, lng: f64, lng_error: f64) -> DecodeGeohashResponse {
        DecodeGeohashResponse {
            lat,
            lat_error,
            lng,
            lng_error,
        }
    }
}

