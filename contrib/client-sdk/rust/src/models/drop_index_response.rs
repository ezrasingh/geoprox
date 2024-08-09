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

/// DropIndexResponse : Returns index deletion status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DropIndexResponse {
    /// If true index was deleted
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl DropIndexResponse {
    /// Returns index deletion status
    pub fn new(deleted: bool) -> DropIndexResponse {
        DropIndexResponse {
            deleted,
        }
    }
}

