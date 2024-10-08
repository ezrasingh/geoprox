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

/// GeohashNeighborsResponse : Neighboring geohash regions
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeohashNeighborsResponse {
    /// East
    #[serde(rename = "e")]
    pub e: String,
    /// North
    #[serde(rename = "n")]
    pub n: String,
    /// North East
    #[serde(rename = "ne")]
    pub ne: String,
    /// North West
    #[serde(rename = "nw")]
    pub nw: String,
    /// South
    #[serde(rename = "s")]
    pub s: String,
    /// South East
    #[serde(rename = "se")]
    pub se: String,
    /// South West
    #[serde(rename = "sw")]
    pub sw: String,
    /// West
    #[serde(rename = "w")]
    pub w: String,
}

impl GeohashNeighborsResponse {
    /// Neighboring geohash regions
    pub fn new(e: String, n: String, ne: String, nw: String, s: String, se: String, sw: String, w: String) -> GeohashNeighborsResponse {
        GeohashNeighborsResponse {
            e,
            n,
            ne,
            nw,
            s,
            se,
            sw,
            w,
        }
    }
}

