use crate::shard::GeoShard;
use serde::{Deserialize, Serialize};

/// Object indentifier
pub type ObjectIdentifier = u64;

/// Latitude/longitude coordinate pair
pub type LatLngCoord = [f64; 2];

/// Nearby object
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Neighbor {
    /// Distance in kilometers
    pub distance: f64,
    /// Object key
    pub key: String,
}

#[derive(Debug)]
pub enum GeoShardError {
    IndexAlreadyExists(String),
    IndexNotFound(String),
    GeohashError(geohash::GeohashError),
}

impl std::fmt::Display for GeoShardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoShardError::IndexAlreadyExists(index) => {
                write!(f, "Index '{}' already exists.", index)
            }
            GeoShardError::IndexNotFound(index) => write!(f, "Index '{}' not found.", index),
            GeoShardError::GeohashError(err) => write!(f, "Geohash error: {}", err),
        }
    }
}

impl std::error::Error for GeoShardError {}

/// Configuration settings for geoshard parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeoShardConfig {
    /// Determines the default geohash length for inserts
    pub insert_depth: Option<usize>,
    /// Determines the default geohash length for searches
    pub search_depth: Option<usize>,
    /// Specifies the default number of results returned in range queries
    pub default_count: Option<usize>,
    /// Toggles the default sorting behavior for query results
    pub default_sorted: Option<bool>,
}

impl Default for GeoShardConfig {
    fn default() -> Self {
        GeoShardConfig {
            insert_depth: Some(GeoShard::DEFAULT_DEPTH),
            search_depth: Some(GeoShard::DEFAULT_DEPTH),
            default_count: Some(GeoShard::DEFAULT_COUNT),
            default_sorted: Some(GeoShard::DEFAULT_SORTED),
        }
    }
}

pub type BatchOutput<A, E> = (Vec<(String, A)>, Vec<(String, E)>);
