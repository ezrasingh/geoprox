use crate::metric::HaversineDistance;
use crate::models::{LatLngCoord, Neighbor, ObjectIdentifier};
use ahash::AHasher;
use geohash::GeohashError;
use hashbrown::hash_table::Entry;
use hashbrown::{HashSet, HashTable};
use kiddo::distance_metric::DistanceMetric;
use kiddo::KdTree;
use log::debug;
use patricia_tree::StringPatriciaMap;
use rayon::prelude::*;
use std::hash::{BuildHasher, BuildHasherDefault};

fn build_search_space(
    prefix_tree: &StringPatriciaMap<HashSet<ObjectIdentifier>>,
    subregion_hash: &str,
) -> KdTree<f64, 2> {
    let mut search_tree = KdTree::new();

    // ? locate nearby geohashes and populate spatial index
    debug!("building kd-tree for region: {}", &subregion_hash);

    for (ghash, members) in prefix_tree.iter_prefix(subregion_hash) {
        if let Ok((position, _, _)) = geohash::decode(&ghash) {
            members.iter().for_each(|id: &ObjectIdentifier| {
                debug!("adding object to kd-tree: id={} geohash={}", id, ghash);
                // ? geohash position uses (lng, lat)
                search_tree.add(&[position.y, position.x], *id);
            });
        }
    }

    search_tree
}

#[derive(Clone, Default)]
pub struct SpatialIndex {
    /// maps geohash to common objects
    prefix_tree: StringPatriciaMap<HashSet<ObjectIdentifier>>,
    /// maps object key, geohash to hash id (object id)
    position_map: HashTable<[String; 2]>,
    /// internal hasher
    hasher: BuildHasherDefault<AHasher>,
}

impl SpatialIndex {
    /// determines the maximum number of elements the spatial index can hold before it needs to reallocate.
    pub const MAX_CAPACITY: usize = u16::MAX as usize;

    pub fn new(capacity: usize) -> Self {
        let position_map = {
            if capacity.gt(&Self::MAX_CAPACITY) {
                HashTable::with_capacity(Self::MAX_CAPACITY)
            } else {
                HashTable::with_capacity(capacity)
            }
        };
        Self {
            position_map,
            ..Default::default()
        }
    }

    /// Insert key into index at some geographical location
    pub fn insert(&mut self, key: &str, ghash: &str) {
        let id: ObjectIdentifier = self.hasher.hash_one(key);
        // ? remove object from previous locations
        if let Entry::Occupied(entry) = self.position_map.entry(
            id,
            |[key, _]| key.eq(key),
            |[key, _]| self.hasher.hash_one(key),
        ) {
            let ([_, old_ghash], _) = entry.remove();
            debug!(
                "removing object from previous ghash: id={} geohash={}",
                id, &old_ghash
            );
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(&id);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? update position_map & prefix_tree
        debug!("storing object: id={} key={}", id, key);

        self.position_map
            .insert_unique(id, [key.to_owned(), ghash.into()], |[key, _]| {
                self.hasher.hash_one(key)
            });
        // ? insert current region into prefix tree
        if let Some(members) = self.prefix_tree.get_mut(ghash) {
            members.insert(id);
        } else {
            self.prefix_tree.insert(ghash, HashSet::from([id]));
        };
    }

    /// insert multiple objects at once
    pub fn insert_many(&mut self, objects: impl IntoIterator<Item = (String, String)>) {
        objects
            .into_iter()
            .for_each(|(key, ghash)| self.insert(&key, &ghash));
    }

    /// Remove key from index
    pub fn remove(&mut self, key: &str) -> bool {
        let id: ObjectIdentifier = self.hasher.hash_one(key);
        if let Entry::Occupied(entry) = self.position_map.entry(
            id,
            |[key, _]| key.eq(key),
            |[key, _]| self.hasher.hash_one(key),
        ) {
            let ([_, ghash], _) = entry.remove();
            if let Some(members) = self.prefix_tree.get_mut(&ghash) {
                members.remove(&id);
                if members.is_empty() {
                    self.prefix_tree.remove(&ghash);
                }
            }
        }
        true
    }

    /// remove multiple objects at once
    pub fn remove_many(&mut self, keys: HashSet<String>) -> bool {
        // returns false if any failed
        keys.iter().fold(true, |_, key| self.remove(key))
    }

    pub fn search(
        &self,
        origin: LatLngCoord,
        radius: f64,
        count: usize,
        sorted: bool,
        search_depth: usize,
    ) -> Result<Vec<Neighbor>, GeohashError> {
        if self.position_map.is_empty() {
            return Ok(vec![]);
        }
        let search_region: String = {
            // ? geohash crate uses (lng, lat) convention
            let mut ghash = geohash::encode([origin[1], origin[0]].into(), search_depth)?;
            // ? truncate subregion hash until it contains the radius
            while ghash.len().gt(&1) {
                let (point, _, _) = geohash::decode(&ghash)?;
                if HaversineDistance::dist(&origin, &[point.y, point.x]).gt(&radius) {
                    break;
                } else {
                    ghash.pop();
                }
            }
            ghash
        };

        // ? compute nearest neighbors
        let neighbors: Vec<Neighbor> = build_search_space(&self.prefix_tree, &search_region)
            .nearest_n_within::<HaversineDistance>(&origin, radius, count, sorted)
            .par_iter()
            .filter_map(|node| {
                self.position_map
                    .find(node.item, |[key, _]| {
                        self.hasher.hash_one(key).eq(&node.item)
                    })
                    .map(|[key, _]| Neighbor {
                        distance: node.distance,
                        key: key.to_owned(),
                    })
            })
            .collect();
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    const DEFAULT_DEPTH: usize = 10;

    fn encode_lat_lng([lat, lng]: LatLngCoord, depth: usize) -> String {
        geohash::encode([lng, lat].into(), depth).unwrap()
    }

    #[test]
    fn test_upsert() {
        let mut geo_index = SpatialIndex::default();
        let insert_depth = 6;
        let range = 1.0;
        let origin: LatLngCoord = [0.0, 0.0];
        let key = "test-key";

        // place key within the search area
        geo_index.insert(key, &encode_lat_lng(origin, insert_depth));

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 1);

        // move key out of search area
        geo_index.insert(key, &encode_lat_lng([-70.0, 100.0], insert_depth));

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);

        // move key back into search area
        geo_index.insert(key, &encode_lat_lng(origin, insert_depth));

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut geo_index = SpatialIndex::default();
        let depth = 6;
        let range = 10.0;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert(&"a", &encode_lat_lng(origin, depth));
        geo_index.insert(&"b", &encode_lat_lng(origin, depth));

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 2);

        geo_index.remove(&"a");
        geo_index.remove(&"b");

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_search() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;
        let range = 1000.0;
        let count = 100;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(vec![
            ("a".to_string(), encode_lat_lng([1.0, 0.0], depth)),
            ("b".to_string(), encode_lat_lng([1.0, 1.0], depth)),
            ("c".to_string(), encode_lat_lng([0.0, 1.0], depth)),
            ("d".to_string(), encode_lat_lng([0.0, 0.0], depth)),
            ("e".to_string(), encode_lat_lng([-1., 0.0], depth)),
            ("f".to_string(), encode_lat_lng([-1.0, -1.0], depth)),
            ("g".to_string(), encode_lat_lng([0.0, -1.0], depth)),
            ("h".to_string(), encode_lat_lng([0.0, 0.0], depth)),
        ]);

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();

        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
    }

    #[test]
    fn test_search_count() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;
        let range = 1000.0;
        let count = 5;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(vec![
            ("a".to_string(), encode_lat_lng([1.0, 0.0], depth)),
            ("b".to_string(), encode_lat_lng([1.0, 1.0], depth)),
            ("c".to_string(), encode_lat_lng([0.0, 1.0], depth)),
            ("d".to_string(), encode_lat_lng([0.0, 0.0], depth)),
            ("e".to_string(), encode_lat_lng([-1., 0.0], depth)),
            ("f".to_string(), encode_lat_lng([-1.0, -1.0], depth)),
            ("g".to_string(), encode_lat_lng([0.0, -1.0], depth)),
            ("h".to_string(), encode_lat_lng([0.0, 0.0], depth)),
        ]);

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();
        assert!(res.len() <= count);
    }

    #[test]
    fn test_search_count_unsorted() {
        // ? see, https://github.com/sdd/kiddo/issues/168
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;
        let range = 1000.0;
        let count = 0;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(vec![
            ("a".to_string(), encode_lat_lng([1.0, 0.0], depth)),
            ("b".to_string(), encode_lat_lng([1.0, 1.0], depth)),
            ("c".to_string(), encode_lat_lng([0.0, 1.0], depth)),
            ("d".to_string(), encode_lat_lng([0.0, 0.0], depth)),
            ("e".to_string(), encode_lat_lng([-1., 0.0], depth)),
            ("f".to_string(), encode_lat_lng([-1.0, -1.0], depth)),
            ("g".to_string(), encode_lat_lng([0.0, -1.0], depth)),
            ("h".to_string(), encode_lat_lng([0.0, 0.0], depth)),
        ]);

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_search_sorted() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 10;
        let range = 1000.0;
        let count = 100;
        let sorted = true;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(vec![
            ("a".to_string(), encode_lat_lng([1.0, 0.0], depth)),
            ("b".to_string(), encode_lat_lng([1.0, 1.0], depth)),
            ("c".to_string(), encode_lat_lng([0.0, 1.0], depth)),
            ("d".to_string(), encode_lat_lng([0.0, 0.0], depth)),
            ("e".to_string(), encode_lat_lng([-1., 0.0], depth)),
            ("f".to_string(), encode_lat_lng([-1.0, -1.0], depth)),
            ("g".to_string(), encode_lat_lng([0.0, -1.0], depth)),
            ("h".to_string(), encode_lat_lng([0.0, 0.0], depth)),
        ]);

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();

        let mut sorted_neighbors = res.to_vec();
        sorted_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        assert_eq!(res, sorted_neighbors);
    }

    #[test]
    fn test_capacity() {
        let capacity = i16::MAX;
        let mut geo_index = SpatialIndex::new(capacity as usize);
        let mut rng = rand::thread_rng();
        let depth: usize = 5;
        let count = 100;
        let sorted = true;

        for n in 0..capacity {
            let (lat, lng) = (rng.gen_range(-90f64..90f64), rng.gen_range(-180f64..180f64));
            geo_index.insert(&n.to_string(), &encode_lat_lng([lat, lng], depth));
        }

        let center = [0f64, 0f64];
        let range = 200f64;

        let res = geo_index
            .search(center, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();
        assert!(res.len() <= count);
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
    }
}
