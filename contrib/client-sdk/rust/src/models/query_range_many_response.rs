/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.2
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// QueryRangeManyResponse : Returns indices and object keys found with their distance
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRangeManyResponse {
    /// Contains information about any errors occured during batch search.
    #[serde(rename = "errors")]
    pub errors: std::collections::HashMap<String, String>,
    /// Object keys found within range
    #[serde(rename = "results")]
    pub results: std::collections::HashMap<String, Vec<models::Neighbor>>,
}

impl QueryRangeManyResponse {
    /// Returns indices and object keys found with their distance
    pub fn new(errors: std::collections::HashMap<String, String>, results: std::collections::HashMap<String, Vec<models::Neighbor>>) -> QueryRangeManyResponse {
        QueryRangeManyResponse {
            errors,
            results,
        }
    }
}

