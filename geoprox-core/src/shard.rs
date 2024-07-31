use std::collections::HashMap;

use serde::Deserialize;

use crate::cache::SpatialIndex;
use crate::models::{LatLngCoord, Neighbor};

#[derive(Debug)]
pub enum GeoShardError {
    IndexAlreadyExists(String),
    IndexNotFound(String),
    GeohashError(geohash::GeohashError),
}

/// Configures geoshard parameters
#[derive(Clone, Debug, Deserialize)]
pub struct GeoShardConfig {
    // geohash length used during insert
    insert_depth: usize,
    // initial subregion geohash length used during range query
    search_depth: Option<usize>,
}

impl Default for GeoShardConfig {
    fn default() -> Self {
        GeoShardConfig {
            insert_depth: SpatialIndex::DEFAULT_DEPTH,
            search_depth: None,
        }
    }
}

/// A collection of geospatial indexes stored in-memory
#[derive(Default, Clone)]
pub struct GeoShard {
    cache: HashMap<String, SpatialIndex>,
    config: GeoShardConfig,
}

impl From<GeoShardConfig> for GeoShard {
    fn from(config: GeoShardConfig) -> Self {
        GeoShard {
            config,
            ..Default::default()
        }
    }
}

impl GeoShard {
    /// Adds a new geospatial index to the shard
    pub fn create_index(&mut self, index: &str) -> Result<Option<()>, GeoShardError> {
        if self.cache.contains_key(index) {
            return Err(GeoShardError::IndexAlreadyExists(index.to_string()));
        }
        match self.cache.insert(index.into(), SpatialIndex::default()) {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }

    pub fn drop_index(&mut self, index: &str) {
        self.cache.remove(index);
    }

    /// Adds a key into the specified index at some geographical position
    pub fn insert_key(
        &mut self,
        index: &str,
        key: &str,
        position: &LatLngCoord,
    ) -> Result<String, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            match geohash::encode([position[1], position[0]].into(), self.config.insert_depth) {
                Ok(ghash) => {
                    geo_index.place_resource(key, &ghash);
                    Ok(ghash)
                }
                Err(err) => Err(GeoShardError::GeohashError(err)),
            }
        } else {
            Err(GeoShardError::IndexNotFound(index.into()))
        }
    }

    /// Deletes a key from the specified index
    pub fn remove_key(&mut self, index: &str, key: &str) -> Result<bool, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            Ok(geo_index.remove_resource(key))
        } else {
            Err(GeoShardError::IndexNotFound(index.into()))
        }
    }

    /// Search index for keys within some range
    pub fn query_range(
        &self,
        index: &str,
        origin: LatLngCoord,
        range: &f64,
    ) -> Result<Vec<Neighbor>, GeoShardError> {
        if let Some(geo_index) = self.cache.get(index) {
            match geo_index.search(origin, range, self.config.search_depth) {
                Ok(found) => Ok(found),
                Err(err) => Err(GeoShardError::GeohashError(err)),
            }
        } else {
            Err(GeoShardError::IndexNotFound(index.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_range() {
        let mut shard = GeoShard::default();
        shard.create_index("drivers").unwrap();

        shard.insert_key("drivers", "alice", &[-0.25, 1.0]).unwrap();
        shard.insert_key("drivers", "bob", &[1.0, 0.5]).unwrap();

        let res = shard.query_range("drivers", [0.0, 0.0], &150.0).unwrap();
        println!("found: {:#?}", res);
    }
}
