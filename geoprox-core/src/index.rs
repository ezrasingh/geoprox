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

use std::collections::BTreeSet;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

#[inline]
fn build_search_space(
    prefix_tree: &StringPatriciaMap<HashSet<ObjectIdentifier>>,
    subregion_hash: &str,
) -> KdTree<f64, 2> {
    let mut search_tree = KdTree::new();

    // ? locate nearby geohashes and populate spatial index
    debug!("building kd-tree for region: {}", &subregion_hash);

    for (ghash, members) in prefix_tree.iter_prefix(subregion_hash) {
        if let Ok((position, _, _)) = geohash::decode(&ghash) {
            members.into_iter().for_each(|id: &ObjectIdentifier| {
                debug!("adding object to kd-tree: id={} geohash={}", id, ghash);
                // ? geohash position uses (lng, lat)
                search_tree.add(&[position.y, position.x], *id);
            });
        }
    }

    search_tree
}

#[derive(Clone, Default, Debug)]
pub struct SpatialIndex {
    /// maps geohash to common objects
    prefix_tree: StringPatriciaMap<HashSet<ObjectIdentifier>>,
    /// maps object key, geohash and expiration to hash id (object id)
    objects: HashTable<(Arc<str>, Arc<str>, Option<Instant>)>,
    /// maps expiration key to object key
    expirations: BTreeSet<(Instant, Arc<str>)>,
    /// internal hasher
    hasher: BuildHasherDefault<AHasher>,
}

impl SpatialIndex {
    pub fn new(capacity: usize) -> Self {
        Self {
            objects: HashTable::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn sys_time_objects(&self) -> Vec<(Arc<str>, Arc<str>, Option<SystemTime>)> {
        let sys_now = SystemTime::now();
        let now = Instant::now();
        self.objects
            .clone()
            .into_iter()
            .map(|(key, ghash, ttl)| (key, ghash, ttl.map(|expire_at| sys_now - (now - expire_at))))
            .collect()
    }

    /// Removes expired keys
    pub fn purge(&mut self) {
        let now = (Instant::now(), Arc::from(""));
        let expired_keys: HashSet<Arc<str>> = self
            .expirations
            .range(..=&now)
            .map(|(_, key)| key.clone())
            .collect();
        self.remove_many(expired_keys);
        self.expirations = self.expirations.split_off(&now);
    }

    /// Insert key into index at some geographical location
    pub fn insert(&mut self, key: &str, ghash: &str, expiration: Option<Duration>) {
        let id: ObjectIdentifier = self.hasher.hash_one(key);
        // ? remove object from previous locations
        if let Entry::Occupied(entry) = self.objects.entry(
            id,
            |(key, _, _)| key.eq(key),
            |(key, _, _)| self.hasher.hash_one(key),
        ) {
            let ((key, old_ghash, old_expiration), _) = entry.remove();
            debug!(
                "removing object from previous ghash: id={} geohash={}",
                id, &old_ghash
            );
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(&id);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
            if let Some(expire_key) = old_expiration {
                debug!("removing old expiration for: id={}", id);
                self.expirations.remove(&(expire_key, key));
            }
        }

        // ? update position_map & prefix_tree
        debug!("storing object: id={} key={}", id, key);
        // ? create reference counting containers
        let key: Arc<str> = Arc::from(key);
        let ghash: Arc<str> = Arc::from(ghash);
        let expire_at = expiration.map(|duration| Instant::now() + duration);
        // ? update expiration tree
        if let Some(instant) = expire_at {
            self.expirations.insert((instant, key.clone()));
        }
        // ? insert current region into prefix tree
        if let Some(members) = self.prefix_tree.get_mut(ghash.clone()) {
            members.insert(id);
        } else {
            self.prefix_tree.insert(ghash.clone(), HashSet::from([id]));
        };
        self.objects
            .insert_unique(id.into(), (key, ghash, expire_at), |(key, _, _)| {
                self.hasher.hash_one(key)
            });
    }

    /// insert multiple objects at once
    pub fn insert_many(
        &mut self,
        objects: impl IntoIterator<Item = (impl ToString, impl ToString)>,
        expiration: Option<Duration>,
    ) {
        objects
            .into_iter()
            .for_each(|(key, ghash)| self.insert(&key.to_string(), &ghash.to_string(), expiration));
    }

    /// Remove key from index
    pub fn remove(&mut self, key: &str) -> bool {
        let id: ObjectIdentifier = self.hasher.hash_one(key);
        if let Entry::Occupied(entry) = self.objects.entry(
            id,
            |(key, _, _)| key.eq(key),
            |(key, _, _)| self.hasher.hash_one(key),
        ) {
            let ((key, ghash, old_expiration), _) = entry.remove();
            if let Some(members) = self.prefix_tree.get_mut(&ghash) {
                members.remove(&id);
                if members.is_empty() {
                    self.prefix_tree.remove(&ghash);
                }
            }
            if let Some(expire_key) = old_expiration {
                self.expirations.remove(&(expire_key, key));
            }
        }
        true
    }

    /// remove multiple objects at once
    pub fn remove_many(&mut self, keys: HashSet<impl ToString>) -> bool {
        // returns false if any failed
        keys.iter()
            .fold(true, |_, key| self.remove(&key.to_string()))
    }

    /// search for objects within the area of some location
    pub fn search(
        &self,
        origin: LatLngCoord,
        radius: f64,
        count: usize,
        sorted: bool,
        search_depth: usize,
    ) -> Result<Vec<Neighbor>, GeohashError> {
        if self.objects.is_empty() || count.eq(&0) {
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
                self.objects
                    .find(node.item, |(key, _, _)| {
                        self.hasher.hash_one(key).eq(&node.item)
                    })
                    .map(|(key, _, _)| Neighbor {
                        distance: node.distance,
                        key: key.clone(),
                    })
            })
            .collect();
        Ok(neighbors)
    }
}

impl PartialEq for SpatialIndex {
    fn eq(&self, other: &Self) -> bool {
        let objects_eq: bool = self
            .objects
            .clone()
            .par_iter()
            .fold(
                || true,
                |result, (key, ghash, ttl)| {
                    let id = other.hasher.hash_one(key);
                    result
                        && other
                            .objects
                            .find(id, |(other_key, other_ghash, other_ttl)| {
                                other_key == key && other_ghash == ghash && other_ttl == ttl
                            })
                            .is_some()
                },
            )
            .reduce(|| true, |result, found| result && found);
        objects_eq && self.expirations == other.expirations && self.hasher == other.hasher
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
    fn can_upsert() {
        let mut geo_index = SpatialIndex::default();
        let insert_depth = 6;
        let range = 1.0;
        let origin: LatLngCoord = [0.0, 0.0];
        let key = "test-key";

        // place key within the search area
        geo_index.insert(key, &encode_lat_lng(origin, insert_depth), None);

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 1);

        // move key out of search area
        geo_index.insert(key, &encode_lat_lng([-70.0, 100.0], insert_depth), None);

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);

        // move key back into search area
        geo_index.insert(key, &encode_lat_lng(origin, insert_depth), None);

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn can_remove() {
        let mut geo_index = SpatialIndex::default();
        let depth = 6;
        let range = 10.0;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert("a", &encode_lat_lng(origin, depth), None);
        geo_index.insert("b", &encode_lat_lng(origin, depth), None);

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 2);

        geo_index.remove("a");
        geo_index.remove("b");

        let res = geo_index
            .search(origin, range, 100, false, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn can_search() {
        let mut geo_index = SpatialIndex::default();
        let range = 1000.0;
        let count = 100;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(
            vec![
                ("a", "s00j8n0"),
                ("b", "s00j8n1"),
                ("c", "s00j8n2"),
                ("d", "s00j8n3"),
                ("e", "s00j8n4"),
                ("f", "s00j8n5"),
                ("g", "s00j8n6"),
                ("h", "s00j8n7"),
            ],
            None,
        );

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();

        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
    }

    #[test]
    fn can_search_count() {
        let mut geo_index = SpatialIndex::default();
        let range = 1000.0;
        let count = 5;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(
            vec![
                ("a", "s00j8n0"),
                ("b", "s00j8n1"),
                ("c", "s00j8n2"),
                ("d", "s00j8n3"),
                ("e", "s00j8n4"),
                ("f", "s00j8n5"),
                ("g", "s00j8n6"),
                ("h", "s00j8n7"),
            ],
            None,
        );

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();
        assert!(res.len() <= count);
    }

    #[test]
    fn can_search_sorted_count() {
        // ? see, https://github.com/sdd/kiddo/issues/168
        let mut geo_index = SpatialIndex::default();
        let range = 1000.0;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(
            vec![
                ("a", "s00j8n0"),
                ("b", "s00j8n1"),
                ("c", "s00j8n2"),
                ("d", "s00j8n3"),
                ("e", "s00j8n4"),
                ("f", "s00j8n5"),
                ("g", "s00j8n6"),
                ("h", "s00j8n7"),
            ],
            None,
        );

        let count = 1;

        let res_sorted = geo_index
            .search(origin, range, count, true, DEFAULT_DEPTH)
            .unwrap();
        let res_unsorted = geo_index
            .search(origin, range, count, false, DEFAULT_DEPTH)
            .unwrap();

        assert_eq!(res_sorted.len(), count);
        assert_eq!(res_unsorted.len(), count);

        let count = 0;

        let res_sorted = geo_index
            .search(origin, range, count, true, DEFAULT_DEPTH)
            .unwrap();
        let res_unsorted = geo_index
            .search(origin, range, count, false, DEFAULT_DEPTH)
            .unwrap();

        assert_eq!(res_sorted.len(), count);
        assert_eq!(res_unsorted.len(), count);
    }

    #[test]
    fn can_search_sorted() {
        let mut geo_index = SpatialIndex::default();
        let range = 1000.0;
        let count = 100;
        let sorted = true;
        let origin: LatLngCoord = [0.0, 0.0];

        geo_index.insert_many(
            vec![
                ("a", "s00j8n0"),
                ("b", "s00j8n1"),
                ("c", "s00j8n2"),
                ("d", "s00j8n3"),
                ("e", "s00j8n4"),
                ("f", "s00j8n5"),
                ("g", "s00j8n6"),
                ("h", "s00j8n7"),
            ],
            None,
        );

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();

        let mut sorted_neighbors = res.to_vec();
        sorted_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        assert_eq!(res, sorted_neighbors);
    }

    #[test]
    fn can_capacity() {
        let capacity = u16::MAX;
        let mut geo_index = SpatialIndex::new(capacity as usize);
        let mut rng = rand::thread_rng();
        let depth: usize = 5;
        let count = 100;
        let sorted = true;

        for n in 0..capacity {
            let (lat, lng) = (rng.gen_range(-90f64..90f64), rng.gen_range(-180f64..180f64));
            geo_index.insert(&n.to_string(), &encode_lat_lng([lat, lng], depth), None);
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

    #[test]
    fn can_purge() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;
        let range = 1000.0;
        let count = 2;
        let sorted = false;
        let origin: LatLngCoord = [0.0, 0.0];

        let duration = Duration::from_millis(500);

        geo_index.insert("a", &encode_lat_lng(origin, depth), Some(duration));
        geo_index.insert("b", &encode_lat_lng(origin, depth), Some(duration));

        std::thread::sleep(duration);

        geo_index.purge();

        let res = geo_index
            .search(origin, range, count, sorted, DEFAULT_DEPTH)
            .unwrap();
        assert_eq!(res.len(), 0);
    }
}
