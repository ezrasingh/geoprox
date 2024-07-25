use crate::metric::HaversineDistance;
use crate::models::{LatLngCoord, Neighbor, ResourceIdentifier};
use geohash::GeohashError;
use kiddo::distance_metric::DistanceMetric;
use kiddo::KdTree;
use patricia_tree::StringPatriciaMap;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

fn generate_id(resource_key: &str) -> ResourceIdentifier {
    let mut s = DefaultHasher::new();
    resource_key.hash(&mut s);
    s.finish()
}

#[derive(Clone, Default)]
pub struct SpatialIndex<K = String> {
    /// maps geohash to common resources
    prefix_tree: StringPatriciaMap<HashSet<ResourceIdentifier>>,
    /// maps resource to geohash
    position_map: HashMap<ResourceIdentifier, String>,
    /// maps resource hash to resource name
    resource_map: HashMap<ResourceIdentifier, K>,
}

impl SpatialIndex {
    /// depth 6 corresponds to ~1kmx1km region
    pub const DEFAULT_DEPTH: usize = 6;

    pub fn new(capacity: usize) -> Self {
        SpatialIndex {
            position_map: HashMap::with_capacity(capacity),
            resource_map: HashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn place_resource(&mut self, resource_key: &str, ghash: &str) {
        let resource = &generate_id(resource_key);
        dbg!("storing resource: ", resource, resource_key);
        self.resource_map.insert(*resource, resource_key.into());
        if let Some(old_ghash) = self.position_map.insert(*resource, ghash.into()) {
            dbg!(
                "removing resource from previous ghash: ",
                resource,
                &old_ghash
            );
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(resource);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? insert current region into prefix tree
        if let Some(members) = self.prefix_tree.get_mut(&ghash) {
            members.insert(*resource);
        } else {
            let mut members: HashSet<ResourceIdentifier> = HashSet::new();
            members.insert(*resource);
            self.prefix_tree.insert(&ghash, members);
        };
    }

    pub fn remove_resource(&mut self, resource_key: &str) -> bool {
        let resource = &generate_id(resource_key);

        self.resource_map.remove(resource);
        match self.position_map.remove(resource) {
            Some(ghash) => self.prefix_tree.get_mut(ghash).unwrap().remove(resource),
            None => true,
        }
    }

    fn build_search_tree(&self, subregion_hash: &str) -> KdTree<f64, 2> {
        let mut search_tree = KdTree::with_capacity(self.resource_map.len());
        // ? locate nearby geohashes and populate spatial index
        dbg!("building spatial index for region: ", &subregion_hash);

        let subtree: patricia_tree::GenericPatriciaMap<String, HashSet<u64>> =
            self.prefix_tree.clone().split_by_prefix(subregion_hash);

        subtree.iter().for_each(|(ghash, members)| {
            dbg!("found resources nearby:", &ghash, &members);
            if let Ok((position, _, _)) = geohash::decode(&ghash) {
                members.iter().for_each(|resource| {
                    dbg!("adding resource to spatial index", &resource, &ghash);
                    // ! fixme
                    search_tree.add(&[position.x, position.y], *resource);
                });
            }
        });

        dbg!("search tree size: ", search_tree.size());
        search_tree
    }

    pub fn search(
        &self,
        position: LatLngCoord,
        radius: &f64,
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
            while ghash.len() > 1 {
                let (point, _, _) = geohash::decode(&ghash)?;
                if HaversineDistance::dist(&origin, &[point.x, point.y]).gt(radius) {
                    break;
                } else {
                    ghash.pop();
                }
            }
            ghash
        };

        // ? compute nearest neighbors
        let neighbors: Vec<Neighbor> = self
            .build_search_tree(&search_region)
            .within_unsorted_iter::<HaversineDistance>(&position, *radius)
            .map(|node| Neighbor {
                distance: node.distance,
                key: self.resource_map.get(&node.item).unwrap().to_string(),
            })
            .collect();
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
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
        let res = geo_index.search(origin, &range, None).unwrap();
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
        let mut geo_index = SpatialIndex::new(1_000_000);
        let mut rng = rand::thread_rng();
        let depth: usize = 5;
        for n in 0..u16::MAX {
            let (lat, lng) = (rng.gen_range(-90f64..90f64), rng.gen_range(-180f64..180f64));
            geo_index.place_resource(
                &n.to_string(),
                &geohash::encode([lng, lat].into(), depth).unwrap(),
            );
        }

        let center = [0f64, 0f64];
        let range = 200f64;
        let res = geo_index.search(center, &range, None).unwrap();
        res.iter().for_each(|neighbor| {
            assert!(neighbor.distance <= range);
        });
        println!("##########");
        println!("Riders within {}km from ({:?}): {:#?}", range, center, res);
    }
}
