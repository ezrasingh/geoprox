/*
 * geoprox-server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// QueryRange : Arguments for range query
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRange {
    /// latitude
    #[serde(rename = "lat")]
    pub lat: f64,
    /// longitude
    #[serde(rename = "lng")]
    pub lng: f64,
    /// search radius in kilometers
    #[serde(rename = "range")]
    pub range: i32,
}

impl QueryRange {
    /// Arguments for range query
    pub fn new(lat: f64, lng: f64, range: i32) -> QueryRange {
        QueryRange {
            lat,
            lng,
            range,
        }
    }
}

