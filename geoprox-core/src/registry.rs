use crate::models::{LatLongCoord, UserIdentifier, Neighbor};
use geohash::GeohashError;
use kiddo::distance_metric::DistanceMetric;
use kiddo::{KdTree, SquaredEuclidean};
use patricia_tree::StringPatriciaMap;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct PositionRegistry {
    pub prefix_tree: StringPatriciaMap<HashSet<UserIdentifier>>,
    pub position_map: HashMap<UserIdentifier, String>,
}

impl Default for PositionRegistry {
    fn default() -> Self {
        PositionRegistry {
            prefix_tree: StringPatriciaMap::new(),
            position_map: HashMap::new(),
        }
    }
}

impl PositionRegistry {
    const KM_CONVERSION_FACTOR: f64 = 1.0;
    
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

    pub fn add_contract(&mut self, position: &LatLongCoord, within: &f64, precision: &usize) -> Result<Vec<Neighbor<f64>>,GeohashError> {
        // todo! - add queue for each user
        self.search(*position, within, precision)
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

    fn search(
        &self,
        position: LatLongCoord,
        within: &f64,
        precision: &usize
    ) -> Result<Vec<Neighbor<f64>>, GeohashError> {
        let radius = within / Self::KM_CONVERSION_FACTOR;
        
        let mut subregion_hash = geohash::encode(position.into(), *precision)?;
        
        {
        let mut d = 0.0;
            // ? truncate subregion hash until it contains our radius
            while subregion_hash.len() > 1 && d < radius {
                subregion_hash.pop();
                let (point, _, _) = geohash::decode(&subregion_hash)?;
                d = SquaredEuclidean::dist(&position, &[point.x, point.y]);
            }
        }
        dbg!("searching subregion: ", &subregion_hash);

        let mut spatial_index = KdTree::new();
        // ? locate nearby geohashes and populate spatial index
        dbg!("prefix tree size: ", self.prefix_tree.len());

        let subtree = self.prefix_tree.clone().split_by_prefix(subregion_hash);

        subtree.iter().for_each(|(ghash, members)| {
            dbg!("found nearby riders:", &ghash, members);
            let (position, _, _) = geohash::decode(&ghash).unwrap();
            members.iter().for_each(|uid| {
                dbg!("adding user to spatial index", uid);
                spatial_index.add(&[position.x, position.y], *uid);
            });
        });

        // ? compute nearest neighbors
        dbg!("spatial index size: ", spatial_index.size());
        let neighbors: Vec<Neighbor<f64>> = spatial_index
            .within_unsorted_iter::<SquaredEuclidean>(&position, radius)
            .map(|node| Neighbor {
                distance: node.distance * Self::KM_CONVERSION_FACTOR,
                uid: node.item,
            })
            .collect();
        Ok(neighbors)
    }
}

#[cfg(test)]
mod test {
    use crate::registry::PositionRegistry;

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
        let radius = &2500.0; // km
        let res = registry.add_contract(center, radius, &10);
        println!("riders within {}km from ({:?}): {:#?}", radius, center, res);
    }
}
