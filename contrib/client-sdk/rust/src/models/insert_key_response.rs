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

/// InsertKeyResponse : Returns key and geohash
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertKeyResponse {
    /// Geohash encoded latitude/longitude
    #[serde(rename = "geohash")]
    pub geohash: String,
    /// Resource key
    #[serde(rename = "key")]
    pub key: String,
}

impl InsertKeyResponse {
    /// Returns key and geohash
    pub fn new(geohash: String, key: String) -> InsertKeyResponse {
        InsertKeyResponse {
            geohash,
            key,
        }
    }
}

