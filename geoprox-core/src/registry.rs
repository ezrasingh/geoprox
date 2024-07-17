use crate::models::{LatLongCoord, UserIdentifier, Neighbor};
use geohash::GeohashError;
use kiddo::{KdTree, SquaredEuclidean};
use patricia_tree::StringPatriciaMap;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct PositionRegistry {
    pub prefix_tree: StringPatriciaMap<HashSet<UserIdentifier>>,
    pub position_map: HashMap<UserIdentifier, String>,
    pub capacity: usize,
}

impl Default for PositionRegistry {
    fn default() -> Self {
        PositionRegistry {
            prefix_tree: StringPatriciaMap::new(),
            position_map: HashMap::new(),
            capacity: 128,
        }
    }
}

impl PositionRegistry {
    // Constants for approximate degrees per meter, radius of the Earth, and conversion factor
    const EARTH_RADIUS_METERS: f64 = 6_371_000.0; // Mean radius of the Earth in meters
    const MEAN_CURVATURE: f64 = Self::EARTH_RADIUS_METERS * std::f64::consts::PI / 180.0;
    const DEGREES_PER_METER: f64 = 1.0 / Self::MEAN_CURVATURE; // Average degrees per meter
    const METER_CONVERSION_FACTOR: f64 = 1.40625; // Approximate conversion factor for degrees per meter

    
    pub fn new() -> Self {
        PositionRegistry::default()
    }
    pub fn store_user(&mut self, uid: &UserIdentifier, position: &LatLongCoord, precision: &usize) -> Result<String, GeohashError> {
        let ghash = geohash::encode(
            geohash::Coord {
                x: position[0],
                y: position[1],
            },
            *precision,
        )?;
        if let Some(old_ghash) = self.position_map.insert(*uid, ghash.clone()) {
            // ? remove rider from previous region
            dbg!("removing rider from previous region: ", uid, &old_ghash);
            if let Some(prev_members) = self.prefix_tree.get_mut(&old_ghash) {
                prev_members.remove(uid);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? insert current region in prefix tree
        if let Some(members) = self.prefix_tree.get_mut(&ghash) {
            members.insert(*uid);
        } else {
            let mut members: HashSet<UserIdentifier> = HashSet::new();
            members.insert(*uid);
            self.prefix_tree.insert(&ghash, members);
        };
        Ok(ghash)
    }

    pub fn add_contract(&mut self, position: &LatLongCoord, within: &f64) -> Result<Vec<Neighbor<f64>>,GeohashError> {
        // todo! - add queue for each user
        self.search(*position, within)
    }

    pub fn remove_user(&mut self, uid: &UserIdentifier) -> bool {
        match self.position_map.remove(uid) {
            Some(ghash) => self
                .prefix_tree
                .get_mut(ghash)
                .unwrap()
                .remove(uid),
            None => true,
        }
    }

    fn build_spatial_index(&self, subregion_hash: &str) -> KdTree<f64, 2> {
        let mut spatial_index = KdTree::with_capacity(self.capacity);
        // ? locate nearby geohashes and populate spatial index
        dbg!("building spatial index for: ", subregion_hash);

        let subtree = self.prefix_tree.clone().split_by_prefix(subregion_hash);

        subtree.iter().for_each(|(ghash, members)| {
            dbg!("found nearby riders:", &ghash, members);
            let (position, _, _) = geohash::decode(&ghash).unwrap();
            members.iter().for_each(|uid| {
                dbg!("adding user to spatial index", uid);
                spatial_index.add(&[position.x, position.y], *uid);
            });
        });

        dbg!("spatial index size: ", spatial_index.size());
        spatial_index
    }

    fn search(
        &self,
        position: LatLongCoord,
        radius: &f64,
    ) -> Result<Vec<Neighbor<f64>>, GeohashError> {
        let precision: usize = {
            // Calculate the length of the geohash area in degrees latitude
            let max_length = 2.0 * radius * Self::DEGREES_PER_METER / Self::METER_CONVERSION_FACTOR;

            // Calculate the number of bits needed to cover the geohash
            let bits_needed = max_length.log2().ceil();

            // Ensure the precision is at least 1 (the smallest geohash precision)
            bits_needed.max(1.0) as usize
        };

        let subregion_hash = geohash::encode(position.into(), precision)?;
        
        // ? compute nearest neighbors
        let neighbors: Vec<Neighbor<f64>> = self.build_spatial_index(&subregion_hash)
            .within_unsorted_iter::<SquaredEuclidean>(&position, *radius)
            .map(|node| Neighbor {
                distance: node.distance / Self::METER_CONVERSION_FACTOR,
                uid: node.item,
            })
            .collect();
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
    use crate::registry::PositionRegistry;
    use rand::prelude::*;

    #[test]
    fn sandbox() {
        let mut registry = PositionRegistry::default();

        let _ = registry.store_user(&0, &[1.0, 0.0], &10);
        let _ = registry.store_user(&1, &[1.0, 1.0], &10);
        let _ = registry.store_user(&2, &[0.0, 1.0], &10);
        let _ = registry.store_user(&3, &[0.0, 0.0], &10);
        let _ = registry.store_user(&4, &[-1.0, 0.0], &10);
        let _ = registry.store_user(&5, &[-1.0, -1.0], &10);
        let _ = registry.store_user(&6, &[0.0, -1.0], &10);
        let _ = registry.store_user(&7, &[-0.0, 0.0], &10);
        let center = &[0.0, 0.0];
        let radius = &1.0; // m
        let res = registry.add_contract(center, radius);
        println!("riders within {}km from ({:?}): {:#?}", radius, center, res);
    }

    #[test]
    fn stress_test() {
        let mut registry = PositionRegistry::default();
        let mut rng = rand::thread_rng();
        let scale = 100;
        let precision = 10;
        for n in 0..0x10000 {
            let (x, y) = (rng.gen_range(0..scale), rng.gen_range(0..scale));
            let _ = registry.store_user(&n, &[x.into(), y.into()], &precision);
        }

        let center = [0.0, 0.0];
        let radius = 5.0;
        let res = registry.add_contract(&center, &radius);
        println!("riders within {}km from ({:?}): {:#?}", radius, center, res);
    }
}
