use crate::metric::HaversineDistance;
use crate::models::{LatLngCoord, Neighbor, ResourceIdentifier};
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
    prefix_tree: &StringPatriciaMap<HashSet<ResourceIdentifier>>,
    subregion_hash: &str,
) -> KdTree<f64, 2> {
    let mut search_tree = KdTree::new();

    // ? locate nearby geohashes and populate spatial index
    debug!("building spatial index for region: {}", &subregion_hash);

    for (ghash, members) in prefix_tree.iter_prefix(subregion_hash) {
        debug!(
            "found resources nearby: geohash={} members={:#?}",
            &ghash, &members
        );
        if let Ok((position, _, _)) = geohash::decode(&ghash) {
            members.iter().for_each(|resource| {
                debug!(
                    "adding resource to spatial index: id={} geohash={}",
                    &resource, &ghash
                );
                search_tree.add(&[position.x, position.y], *resource);
            });
        }
    }

    search_tree
}

#[derive(Clone, Default)]
pub struct SpatialIndex<K = String> {
    /// maps geohash to common resources
    prefix_tree: StringPatriciaMap<HashSet<ResourceIdentifier>>,
    /// maps resource key, geohash to hash id (resource id)
    position_map: HashTable<(K, String)>,
    /// internal hasher
    hasher: BuildHasherDefault<AHasher>,
}

impl SpatialIndex {
    /// depth 6 corresponds to ~1kmx1km region
    pub const DEFAULT_DEPTH: usize = 6;
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

    /// Upsert key into index at some geographical location
    pub fn place_resource(&mut self, resource_key: &str, ghash: &str) {
        let resource_id: ResourceIdentifier = self.hasher.hash_one(resource_key);
        // ? remove resource from previous locations
        if let Entry::Occupied(entry) = self.position_map.entry(
            resource_id,
            |(key, _)| key.eq(resource_key),
            |(key, _)| self.hasher.hash_one(key),
        ) {
            let ((_, old_ghash), _) = entry.remove();
            debug!(
                "removing resource from previous ghash: id={} geohash={}",
                resource_id, &old_ghash
            );
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(&resource_id);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? update position_map & prefix_tree
        debug!("storing resource: id={} key={}", resource_id, resource_key);

        self.position_map.insert_unique(
            resource_id,
            (resource_key.to_owned(), ghash.into()),
            |(key, _)| self.hasher.hash_one(key),
        );
        // ? insert current region into prefix tree
        if let Some(members) = self.prefix_tree.get_mut(ghash) {
            members.insert(resource_id);
        } else {
            self.prefix_tree.insert(ghash, HashSet::from([resource_id]));
        };
    }

    /// Remove key from index
    pub fn remove_resource(&mut self, resource_key: &str) -> bool {
        let resource_id = self.hasher.hash_one(resource_key);
        if let Entry::Occupied(entry) = self.position_map.entry(
            resource_id,
            |(key, _)| key.eq(resource_key),
            |(key, _)| self.hasher.hash_one(key),
        ) {
            let ((_, ghash), _) = entry.remove();
            if let Some(members) = self.prefix_tree.get_mut(&ghash) {
                members.remove(&resource_id);
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
        let neighbors: Vec<Neighbor> = build_search_space(&self.prefix_tree, &search_region)
            .nearest_n_within::<HaversineDistance>(&position, radius, count, sorted)
            .par_iter()
            .map(|node| {
                let (resource_key, _) = self
                    .position_map
                    .find(node.item, |(key, _)| {
                        self.hasher.hash_one(key).eq(&node.item)
                    })
                    .unwrap();
                Neighbor {
                    distance: node.distance,
                    key: resource_key.to_owned(),
                }
            })
            .collect();
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
    use std::i32;

    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_search() {
        let mut geo_index = SpatialIndex::default();
        let depth: usize = 6;

        geo_index.place_resource(&"a", &geohash::encode([1.0, 0.0].into(), depth).unwrap());
        geo_index.place_resource(&"b", &geohash::encode([1.0, 1.0].into(), depth).unwrap());
        geo_index.place_resource(&"c", &geohash::encode([0.0, 1.0].into(), depth).unwrap());
        geo_index.place_resource(&"d", &geohash::encode([0.0, 0.0].into(), depth).unwrap());
        geo_index.place_resource(&"e", &geohash::encode([-1., 0.0].into(), depth).unwrap());
        geo_index.place_resource(&"f", &geohash::encode([-1.0, -1.0].into(), depth).unwrap());
        geo_index.place_resource(&"g", &geohash::encode([0.0, -1.0].into(), depth).unwrap());
        geo_index.place_resource(&"h", &geohash::encode([0.0, 0.0].into(), depth).unwrap());

        let range = 1.0; // km
        let origin: LatLngCoord = [0f64, 0f64];
        let res = geo_index.search(origin, range, 100, false, None).unwrap();
        assert_eq!(res.len(), 2);
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
        println!("##########");
        println!(
            "Riders found within {}km from {:?}: {:#?}",
            range, origin, res
        );
    }

    #[test]
    fn test_capacity() {
        let capacity = i32::MAX;
        let mut geo_index = SpatialIndex::new(capacity as usize);
        let mut rng = rand::thread_rng();
        let depth: usize = 5;
        println!("loading data set...");
        for n in 0..capacity {
            let (lat, lng) = (rng.gen_range(-90f64..90f64), rng.gen_range(-180f64..180f64));
            geo_index.place_resource(
                &n.to_string(),
                &geohash::encode([lng, lat].into(), depth).unwrap(),
            );
        }

        let center = [0f64, 0f64];
        let range = 200f64;
        println!("searching area...");
        let res = geo_index.search(center, range, 100, true, None).unwrap();
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
        println!("##########");
        println!("Riders within {}km from ({:?}): {:#?}", range, center, res);
    }
}
