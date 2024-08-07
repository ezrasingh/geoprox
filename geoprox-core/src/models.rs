use crate::cache::SpatialIndex;
use serde::{Deserialize, Serialize};

/// Object indentifier
pub type ObjectIdentifier = u64;

/// Latitude/longitude coordinate pair
pub type LatLngCoord = [f64; 2];

/// Nearby object
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
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

/// Configures geoshard parameters
#[derive(Clone, Debug, Deserialize)]
pub struct GeoShardConfig {
    // geohash length used during insert
    pub insert_depth: usize,
    // initial subregion geohash length used during range query
    pub search_depth: Option<usize>,
}

impl Default for GeoShardConfig {
    fn default() -> Self {
        GeoShardConfig {
            insert_depth: SpatialIndex::DEFAULT_DEPTH,
            search_depth: None,
        }
    }
}

pub type BatchOutput<A, E> = (Vec<(String, A)>, Vec<(String, E)>);
