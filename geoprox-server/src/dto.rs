use std::collections::HashMap;

use geoprox_core::models::{LatLngCoord, Neighbor};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};

/// Returns geohash decoded as latitude/longitude with precision errors
#[derive(Serialize, ToSchema, ToResponse)]
pub struct DecodeGeohashResponse {
    /// Latitude
    pub lat: f64,
    /// Latitude error
    pub lat_error: f64,
    /// Longitude
    pub lng: f64,
    /// Longitude error
    pub lng_error: f64,
}

/// Arguments for encoding latitude/longitude as geohash
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct EncodeLatLng {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Determines geohash length
    #[schema(minimum = 1, maximum = 10)]
    pub depth: usize,
}

/// Returns geohash encoded latitude/longitude
#[derive(Serialize, ToSchema, ToResponse)]
pub struct EncodeLatLngResponse {
    pub geohash: String,
}

/// Neighboring geohash regions
#[derive(Serialize, ToSchema, ToResponse)]
pub struct GeohashNeighborsResponse {
    /// South West
    pub sw: String,
    /// South
    pub s: String,
    /// South East
    pub se: String,
    /// West
    pub w: String,
    /// East
    pub e: String,
    /// North West
    pub nw: String,
    /// North
    pub n: String,
    /// North East
    pub ne: String,
}

impl From<geoprox_core::geohash::Neighbors> for GeohashNeighborsResponse {
    fn from(value: geoprox_core::geohash::Neighbors) -> Self {
        Self {
            sw: value.sw,
            s: value.s,
            se: value.se,
            w: value.w,
            e: value.e,
            nw: value.nw,
            n: value.n,
            ne: value.ne,
        }
    }
}

/// Returns index creation status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct CreateIndexResponse {
    /// If true index was created
    pub created: bool,
    /// If true index alredy exist
    pub existed: bool,
}

/// Arguments for inserting a key
#[derive(Deserialize, ToSchema)]
pub struct InsertKey {
    /// Object key
    pub key: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// The time-to-live (TTL) for this key, in seconds
    pub ttl: Option<u64>,
}

/// Returns key and geohash
#[derive(Serialize, ToSchema, ToResponse)]
pub struct InsertKeyResponse {
    /// Object key
    pub key: String,
    /// Geohash encoded latitude/longitude
    pub geohash: String,
}

/// Arguments for inserting multiple keys
#[derive(Deserialize, ToSchema)]
pub struct InsertKeyBatch {
    /// Object key
    pub keys: Vec<InsertKey>,
    /// The time-to-live (TTL) for these keys, in seconds
    pub ttl: Option<u64>,
    // Insert keys in the order they were received.
    pub preserve_order: bool,
}

impl From<InsertKeyBatch> for Vec<(String, LatLngCoord)> {
    fn from(val: InsertKeyBatch) -> Self {
        val.keys
            .iter()
            .map(|insert| (insert.key.to_owned(), [insert.lat, insert.lng]))
            .collect()
    }
}

/// Returns results and errors of batch key insert
#[derive(Serialize, ToSchema, ToResponse)]
pub struct InsertKeyBatchResponse {
    /// Object keys that have been inserted in the index and their geohash.
    pub results: HashMap<String, String>,
    /// Contains information about which keys failed to be inserted and the associated error details.
    pub errors: HashMap<String, String>,
}

/// Arguments for removing a key
#[derive(Deserialize, ToSchema)]
pub struct RemoveKey {
    /// Object key
    pub key: String,
}

/// Returns key and deletion status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct RemoveKeyResponse {
    /// Object key
    pub key: String,
    /// If true key was removed
    pub deleted: bool,
}

/// Arguments for removing multiple keys
#[derive(Deserialize, ToSchema)]
pub struct RemoveKeyBatch {
    /// Object key
    pub keys: Vec<String>,
}

/// Returns batch key deletion status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct RemoveKeyBatchResponse {
    /// If true all keys were removed
    pub deleted: bool,
}

/// Returns index deletion status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct DropIndexResponse {
    /// If true index was deleted
    pub deleted: bool,
}

/// Arguments for range query
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct QueryRange {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Search radius in kilometers
    #[schema(minimum = 0, maximum = 0xFFFF)]
    pub range: u16,
    /// Maximum number of neighbors that can be returned (default 100)
    #[schema(minimum = 1, maximum = 0xFFFF)]
    pub count: Option<usize>,
    /// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    pub sorted: Option<bool>,
}

/// Returns object keys found with their distance
#[derive(Debug, Serialize, ToSchema, ToResponse, Deserialize)]
pub struct QueryRangeResponse {
    /// Object keys found within range
    pub found: Vec<Neighbor>,
}

/// Arguments for group range query
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct QueryRangeMany {
    /// List of indices to search
    pub indices: Vec<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Search radius in kilometers
    #[schema(minimum = 0, maximum = 0xFFFF)]
    pub range: u16,
    /// Maximum number of neighbors that can be returned (default 100)
    #[schema(minimum = 1, maximum = 0xFFFF)]
    pub count: Option<usize>,
    /// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    pub sorted: Option<bool>,
}

/// Returns indices and object keys found with their distance
#[derive(Serialize, ToSchema, ToResponse)]
pub struct QueryRangeManyResponse {
    /// Object keys found within range
    pub results: HashMap<String, Vec<Neighbor>>,
    /// Contains information about any errors occured during batch search.
    pub errors: HashMap<String, String>,
}

/// Returns structured error message
#[derive(Serialize, ToSchema, ToResponse)]
pub struct AppErrorResponse {
    /// Error details
    pub message: String,
}
