use std::collections::HashMap;
use crate::models::{LatLngCoord, Neighbor};
use crate::cache::SpatialIndex;

#[derive(Clone)]
pub struct GeoShardConfig{
    // geohash length used during insert
    insert_depth: usize
}

impl Default for GeoShardConfig {
    fn default() -> Self {
        GeoShardConfig {
            insert_depth: 6
        }
    }
}

#[derive(Default, Clone)]
pub struct GeoShard{
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
    pub fn create_index(&mut self, index: &str) {
        if self.cache.contains_key(index) {
            panic!("geoshard already contains the index: '{}'", index);
        }
        self.cache.insert(index.into(), SpatialIndex::default());
    }

    pub fn drop_index(&mut self, index: &str) {
        self.cache.remove(index);
    }

    pub fn insert_key(&mut self, index: &str, key: &str, position: &LatLngCoord) -> Result<String, geohash::GeohashError> {
        let ghash = geohash::encode([position[1], position[0]].into(), self.config.insert_depth)?;
        if let Some(geo_index) = self.cache.get_mut(index) {
            geo_index.place_resource(key, &ghash);
        } else {
            panic!("geoshard does not contain the index: '{}'", index);
        }
        Ok(ghash)
    }
    pub fn remove_key(&mut self, index: &str, key: &str) {
        if let Some(geo_index) = self.cache.get_mut(index) {
            geo_index.remove_resource(key);
        } else {
            panic!("geoshard does not contain the index: '{}'", index);
        }
    }
    pub fn query_range(&self, index: &str, origin: LatLngCoord, range: &f64) -> Result<Vec<Neighbor>, geohash::GeohashError> {
        if let Some(geo_index) = self.cache.get(index) {
           geo_index.search(origin, range)
        } else {
            panic!("geoshard does not contain the index: '{}'", index);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::shard::GeoShard;

    #[test]
    fn sandbox() {
        let mut shard = GeoShard::default();
        shard.create_index("drivers");

        shard.insert_key("drivers", "alice", &[-0.25, 1.0]).unwrap();
        shard.insert_key("drivers", "bob", &[1.0, 0.5]).unwrap();

        let res = shard.query_range("drivers", [0.0, 0.0], &150.0).unwrap();
        println!("found: {:#?}", res);

    }
}
