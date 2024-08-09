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

/// RemoveKeyBatchResponse : Returns batch key deletion status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveKeyBatchResponse {
    /// If true all keys were removed
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl RemoveKeyBatchResponse {
    /// Returns batch key deletion status
    pub fn new(deleted: bool) -> RemoveKeyBatchResponse {
        RemoveKeyBatchResponse {
            deleted,
        }
    }
}

