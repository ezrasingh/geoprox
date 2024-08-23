/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// QueryRangeResponse : Returns object keys found with their distance
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRangeResponse {
    /// Object keys found within range
    #[serde(rename = "found")]
    pub found: Vec<models::Neighbor>,
}

impl QueryRangeResponse {
    /// Returns object keys found with their distance
    pub fn new(found: Vec<models::Neighbor>) -> QueryRangeResponse {
        QueryRangeResponse {
            found,
        }
    }
}

