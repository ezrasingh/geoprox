use geohash::GeohashError;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

use crate::cache::SpatialIndex;
use crate::models::{BatchOutput, GeoShardConfig, GeoShardError, LatLngCoord, Neighbor};

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
    /// depth 6 corresponds to ~1kmx1km region
    pub const DEFAULT_DEPTH: usize = 6;
    pub const DEFAULT_COUNT: usize = 100;
    pub const DEFAULT_SORTED: bool = false;

    /// Adds a new geospatial index to the shard
    pub fn create_index(&mut self, index: &str) -> Result<Option<()>, GeoShardError> {
        if self.cache.contains_key(index) {
            return Err(GeoShardError::IndexAlreadyExists(index.to_owned()));
        }
        match self.cache.insert(index.into(), SpatialIndex::default()) {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }

    /// Deletes index from the shard
    pub fn drop_index(&mut self, index: &str) {
        self.cache.remove(index);
    }

    /// Adds a key into the specified index at some geographical position
    pub fn insert_key(
        &mut self,
        index: &str,
        key: &str,
        [lat, lng]: LatLngCoord,
    ) -> Result<String, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            match geohash::encode(
                [lng, lat].into(),
                self.config.insert_depth.unwrap_or(Self::DEFAULT_DEPTH),
            ) {
                Ok(ghash) => {
                    geo_index.insert(key, &ghash);
                    Ok(ghash)
                }
                Err(err) => Err(GeoShardError::GeohashError(err)),
            }
        } else {
            Err(GeoShardError::IndexNotFound(index.to_owned()))
        }
    }

    /// Adds multiple keys into the specified index at some geographical position
    pub fn insert_many_keys(
        &mut self,
        index: &str,
        objects: Vec<(String, LatLngCoord)>,
        preserve_order: bool,
    ) -> Result<BatchOutput<String, GeohashError>, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            let insert_depth = self.config.insert_depth.unwrap_or(Self::DEFAULT_DEPTH);
            let (bulk, errors): BatchOutput<String, GeohashError> = if !preserve_order {
                // ? use parallel iterator
                objects.into_par_iter().partition_map(|(key, [lat, lng])| {
                    match geohash::encode([lng, lat].into(), insert_depth) {
                        Ok(ghash) => rayon::iter::Either::Left((key, ghash)),
                        Err(err) => rayon::iter::Either::Right((key, err)),
                    }
                })
            } else {
                // ? use sequential iterator
                objects.into_iter().partition_map(|(key, [lat, lng])| {
                    match geohash::encode([lng, lat].into(), insert_depth) {
                        Ok(ghash) => itertools::Either::Left((key, ghash)),
                        Err(err) => itertools::Either::Right((key, err)),
                    }
                })
            };

            geo_index.insert_many(bulk.clone());

            Ok((bulk, errors))
        } else {
            Err(GeoShardError::IndexNotFound(index.to_owned()))
        }
    }

    /// Deletes a key from the specified index
    pub fn remove_key(&mut self, index: &str, key: &str) -> Result<bool, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            Ok(geo_index.remove(key))
        } else {
            Err(GeoShardError::IndexNotFound(index.to_owned()))
        }
    }

    /// Deletes multiple keys from the specified index
    pub fn remove_many_keys(
        &mut self,
        index: &str,
        keys: HashSet<String>,
    ) -> Result<bool, GeoShardError> {
        if let Some(geo_index) = self.cache.get_mut(index) {
            Ok(geo_index.remove_many(keys))
        } else {
            Err(GeoShardError::IndexNotFound(index.to_owned()))
        }
    }

    /// Search index for keys within some range
    pub fn query_range(
        &self,
        index: &str,
        origin: LatLngCoord,
        range: f64,
        count: Option<usize>,
        sorted: Option<bool>,
    ) -> Result<Vec<Neighbor>, GeoShardError> {
        let count = count.unwrap_or(Self::DEFAULT_COUNT);
        if count.eq(&0) {
            return Ok(vec![]);
        }
        if let Some(geo_index) = self.cache.get(index) {
            match geo_index.search(
                origin,
                range,
                count,
                sorted.unwrap_or(Self::DEFAULT_SORTED),
                self.config.search_depth.unwrap_or(Self::DEFAULT_DEPTH),
            ) {
                Ok(found) => Ok(found),
                Err(err) => Err(GeoShardError::GeohashError(err)),
            }
        } else {
            Err(GeoShardError::IndexNotFound(index.to_owned()))
        }
    }

    /// Search multiple indices for keys within some range
    pub fn query_range_many(
        &self,
        indices: HashSet<String>,
        origin: LatLngCoord,
        range: f64,
        count: Option<usize>,
        sorted: Option<bool>,
    ) -> BatchOutput<Vec<Neighbor>, GeoShardError> {
        indices.into_par_iter().partition_map(|index| {
            match self.query_range(&index, origin, range, count, sorted) {
                Ok(found) => itertools::Either::Left((index, found)),
                Err(err) => itertools::Either::Right((index, err)),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_index() {
        let mut shard = GeoShard::default();
        let mock_index = "mock-index";

        let res = shard.create_index(mock_index);
        assert!(res.is_ok());

        let res = shard.create_index(mock_index);
        assert!(res.is_err());
    }

    #[test]
    fn test_drop_index() {
        let mut shard = GeoShard::default();
        let mock_index = "mock-index";

        let res = shard.create_index(mock_index);
        assert!(res.is_ok());

        shard.drop_index(mock_index);

        let res = shard.create_index(mock_index);
        assert!(res.is_ok());
    }

    #[test]
    fn test_query_range() {
        let mut shard = GeoShard::default();
        let mock_index = "mock-index";
        let count = Some(100);
        let sorted = None;
        shard.create_index(mock_index).unwrap();

        shard
            .insert_many_keys(
                mock_index,
                vec![
                    ("a".to_string(), [-0.25, 1.0]),
                    ("b".to_string(), [1.0, 0.5]),
                    ("c".to_string(), [0.0, 0.0]),
                ],
                false,
            )
            .unwrap();

        let res = shard
            .query_range(mock_index, [0.0, 0.0], 150.0, count, sorted)
            .unwrap();
        assert_eq!(res.len(), count.unwrap());
    }

    #[test]
    fn test_query_range_count() {
        let mut shard = GeoShard::default();
        let mock_index = "mock-index";
        let sorted = None;
        shard.create_index(mock_index).unwrap();

        shard
            .insert_many_keys(
                mock_index,
                vec![
                    ("a".to_string(), [0.0, 1.0]),
                    ("b".to_string(), [1.0, 0.5]),
                    ("c".to_string(), [0.0, 0.0]),
                    ("d".to_string(), [0.0, -1.0]),
                    ("e".to_string(), [-1.0, -0.5]),
                    ("f".to_string(), [0.0, 0.0]),
                ],
                false,
            )
            .unwrap();

        {
            let count = Some(100);
            let res = shard
                .query_range(mock_index, [0.0, 0.0], 1000.0, count, sorted)
                .unwrap();

            assert!(res.len() <= count.unwrap());
        }

        {
            let count = Some(0);

            let res = shard
                .query_range(mock_index, [0.0, 0.0], 1000.0, count, sorted)
                .unwrap();

            assert!(res.len() <= count.unwrap());
        }
    }

    #[test]
    fn test_query_range_sorted() {
        let mut shard = GeoShard::default();
        let mock_index = "mock-index";
        let count = Some(100);
        let sorted = Some(true);
        shard.create_index(mock_index).unwrap();

        shard
            .insert_many_keys(
                mock_index,
                vec![
                    ("a".to_string(), [0.0, 1.0]),
                    ("b".to_string(), [1.0, 0.5]),
                    ("c".to_string(), [0.0, 0.0]),
                    ("d".to_string(), [0.0, -1.0]),
                    ("e".to_string(), [-1.0, -0.5]),
                    ("f".to_string(), [0.0, 0.0]),
                ],
                false,
            )
            .unwrap();

        let res = shard
            .query_range(mock_index, [0.0, 0.0], 1000.0, count, sorted)
            .unwrap();

        let mut sorted_neighbors = res.to_vec();

        sorted_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        assert_eq!(res, sorted_neighbors);
    }
}
