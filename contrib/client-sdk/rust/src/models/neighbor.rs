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

/// Neighbor : Nearby object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Neighbor {
    /// Distance in kilometers
    #[serde(rename = "distance")]
    pub distance: f64,
    /// Object key
    #[serde(rename = "key")]
    pub key: String,
}

impl Neighbor {
    /// Nearby object
    pub fn new(distance: f64, key: String) -> Neighbor {
        Neighbor {
            distance,
            key,
        }
    }
}

