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
                search_tree.add(&[position.x, position.y], *id);
            });
        }
    }

    search_tree
}

#[derive(Clone, Default)]
pub struct SpatialIndex<K = String> {
    /// maps geohash to common resources
    prefix_tree: StringPatriciaMap<HashSet<ObjectIdentifier>>,
    /// maps resource key, geohash to hash id (resource id)
    position_map: HashTable<(K, String)>,
    /// internal hasher
    hasher: BuildHasherDefault<AHasher>,
}

impl SpatialIndex {
    /// depth 6 corresponds to ~1kmx1km region
    pub const DEFAULT_DEPTH: usize = 6;
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
            |(key, _)| key.eq(key),
            |(key, _)| self.hasher.hash_one(key),
        ) {
            let ((_, old_ghash), _) = entry.remove();
            debug!(
                "removing resource from previous ghash: id={} geohash={}",
                id, &old_ghash
            );
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(&id);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? update position_map & prefix_tree
        debug!("storing resource: id={} key={}", id, key);

        self.position_map
            .insert_unique(id, (key.to_owned(), ghash.into()), |(key, _)| {
                self.hasher.hash_one(key)
            });
        // ? insert current region into prefix tree
        if let Some(members) = self.prefix_tree.get_mut(ghash) {
            members.insert(id);
        } else {
            self.prefix_tree.insert(ghash, HashSet::from([id]));
        };
    }

    /// Remove key from index
    pub fn remove(&mut self, key: &str) -> bool {
        let id: ObjectIdentifier = self.hasher.hash_one(key);
        if let Entry::Occupied(entry) = self.position_map.entry(
            id,
            |(key, _)| key.eq(key),
            |(key, _)| self.hasher.hash_one(key),
        ) {
            let ((_, ghash), _) = entry.remove();
            if let Some(members) = self.prefix_tree.get_mut(&ghash) {
                members.remove(&id);
                if members.is_empty() {
                    self.prefix_tree.remove(&ghash);
                }
            }
        }
        true
    }

    pub fn search(
        &self,
        position: LatLngCoord,
        radius: f64,
        count: usize,
        sorted: bool,
        initial_depth: Option<usize>,
    ) -> Result<Vec<Neighbor>, GeohashError> {
        if self.position_map.is_empty() {
            return Ok(vec![]);
        }
        let search_region: String = {
            let depth = initial_depth.unwrap_or(Self::DEFAULT_DEPTH);
            let mut ghash = geohash::encode(position.into(), depth)?;
            // ? kiddo uses (lng,lat) for calculations so we flip the LatLngCoord
            let origin = [position[1], position[0]];
            // ? truncate subregion hash until it contains our radius
            while ghash.len().gt(&1) {
                let (point, _, _) = geohash::decode(&ghash)?;
                if HaversineDistance::dist(&origin, &[point.x, point.y]).gt(&radius) {
                    break;
                } else {
                    ghash.pop();
                }
            }
            ghash
        };

        // ? compute nearest neighbors
        let mut neighbors: Vec<Neighbor> = build_search_space(&self.prefix_tree, &search_region)
            .nearest_n_within::<HaversineDistance>(&position, radius, count, sorted)
            .par_iter()
            .filter_map(|node| {
                self.position_map
                    .find(node.item, |(key, _)| {
                        self.hasher.hash_one(key).eq(&node.item)
                    })
                    .map(|(key, _)| Neighbor {
                        distance: node.distance,
                        key: key.to_owned(),
                    })
            })
            .collect();
        if !sorted {
            // ! kiddo does not properly handle count argument when sort is disabled
            // ! see sdd/kiddo#168 - https://github.com/sdd/kiddo/issues/168
            neighbors.truncate(count);
        }
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_upsert() {
        let mut geo_index = SpatialIndex::default();
        let insert_depth = 6;
        let range = 1.0; // km
        let origin: LatLngCoord = [0.0, 0.0];
        let key = "test-key";

        // place key within the search area
        geo_index.insert(key, &geohash::encode(origin.into(), insert_depth).unwrap());

        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 1);

        // move key out of search area
        geo_index.insert(
            key,
            &geohash::encode([-2.0, -2.0].into(), insert_depth).unwrap(),
        );

        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 0);

        // move key back into search area
        geo_index.insert(key, &geohash::encode(origin.into(), insert_depth).unwrap());

        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut geo_index = SpatialIndex::default();
        let depth = 6;
        let range = 10.0; // km
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert(&"a", &geohash::encode(origin.into(), depth).unwrap());
        geo_index.insert(&"b", &geohash::encode(origin.into(), depth).unwrap());

        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 2);

        geo_index.remove(&"a");
        geo_index.remove(&"b");

        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_search() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;
        let range = 1.0; // km
        let count = 100;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert(&"a", &geohash::encode([1.0, 0.0].into(), depth).unwrap());
        geo_index.insert(&"b", &geohash::encode([1.0, 1.0].into(), depth).unwrap());
        geo_index.insert(&"c", &geohash::encode([0.0, 1.0].into(), depth).unwrap());
        geo_index.insert(&"d", &geohash::encode([0.0, 0.0].into(), depth).unwrap());
        geo_index.insert(&"e", &geohash::encode([-1., 0.0].into(), depth).unwrap());
        geo_index.insert(&"f", &geohash::encode([-1.0, -1.0].into(), depth).unwrap());
        geo_index.insert(&"g", &geohash::encode([0.0, -1.0].into(), depth).unwrap());
        geo_index.insert(&"h", &geohash::encode([0.0, 0.0].into(), depth).unwrap());

        let res = geo_index
            .search(origin, range, count, sorted, None)
            .unwrap();
        assert!(res.len() <= count);
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
        println!(
            "Objects found within {}km from {:?}: {:#?}",
            range, origin, res
        );
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
            geo_index.insert(
                &n.to_string(),
                &geohash::encode([lng, lat].into(), depth).unwrap(),
            );
        }

        let center = [0f64, 0f64];
        let range = 200f64;
        println!("searching area...");
        let res = geo_index
            .search(center, range, count, sorted, None)
            .unwrap();
        assert!(res.len() <= count);
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
    }
}
