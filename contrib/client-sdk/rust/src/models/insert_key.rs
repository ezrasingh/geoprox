/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InsertKey : Arguments for inserting a key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertKey {
    /// Resource key
    #[serde(rename = "key")]
    pub key: String,
    /// Latitude
    #[serde(rename = "lat")]
    pub lat: f64,
    /// Longitude
    #[serde(rename = "lng")]
    pub lng: f64,
}

impl InsertKey {
    /// Arguments for inserting a key
    pub fn new(key: String, lat: f64, lng: f64) -> InsertKey {
        InsertKey {
            key,
            lat,
            lng,
        }
    }
}

