use crate::models;
use kiddo::distance_metric::DistanceMetric;
use kiddo::{KdTree, SquaredEuclidean};
use patricia_tree::StringPatriciaMap;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

const KM_CONVERSION_FACTOR: f64 = 4759.8489494771573056620264129372084337922727029429072519352176830475693418667362766156357719841267033536508992127938734206789816;

#[derive(Clone)]
pub struct AppState {
    pub precision: usize,
    pub prefix_tree: StringPatriciaMap<HashSet<u64>>,
    pub position_map: HashMap<u64, String>,
}

pub type SharedState = Arc<RwLock<AppState>>;

impl Default for AppState {
    fn default() -> Self {
        AppState {
            precision: 10,
            prefix_tree: StringPatriciaMap::new(),
            position_map: HashMap::new(),
        }
    }
}

impl AppState {
    pub fn new(precision: usize) -> Self {
        AppState {
            precision: precision,
            prefix_tree: StringPatriciaMap::new(),
            position_map: HashMap::new(),
        }
    }
    pub fn place_rider(&mut self, payload: models::PlaceRider) -> models::PlaceRiderResponse {
        let ghash = geohash::encode(
            geohash::Coord {
                x: payload.position[0],
                y: payload.position[1],
            },
            self.precision,
        )
        .unwrap();
        if let Some(old_ghash) = self.position_map.insert(payload.uid, ghash.clone()) {
            // ? remove rider from previous region
            if let Some(prev_members) = self.prefix_tree.get_mut(old_ghash.clone()) {
                prev_members.remove(&payload.uid);
            } else {
                self.prefix_tree.remove(old_ghash);
            }
        }
        // ? insert current region in prefix tree
        if let Some(members) = self.prefix_tree.get_mut(ghash.clone()) {
            members.insert(payload.uid);
        } else {
            let mut members: HashSet<u64> = HashSet::new();
            members.insert(payload.uid);
            self.prefix_tree.insert(ghash.clone(), members);
        };
        println!(
            "placed rider: ({}, {}) => {:?}",
            payload.uid,
            ghash.clone(),
            payload.position
        );
        models::PlaceRiderResponse { geohash: ghash }
    }

    pub fn place_order(self, payload: models::PlaceOrder) -> models::PlaceOrderResponse {
        let neighbors = self.search(payload.position, payload.distance);
        models::PlaceOrderResponse { riders: neighbors }
    }

    pub fn remove_rider(&mut self, payload: models::RemoveRider) -> models::RemoveRiderResponse {
        let result: bool = match self.position_map.remove(&payload.uid) {
            Some(ghash) => self
                .prefix_tree
                .get_mut(ghash)
                .unwrap()
                .remove(&payload.uid),
            None => true,
        };
        models::RemoveRiderResponse { removed: result }
    }

    fn search(
        self,
        location: models::LatLongCoord,
        within: f64,
    ) -> Vec<models::Neighbor<f64, u64>> {
        let radius = within / KM_CONVERSION_FACTOR;
        let mut subregion_hash = geohash::encode(location.into(), self.precision).unwrap();
        let mut d = 0.0;
        // ? truncate subregion hash until it contains our radius
        while d < radius {
            let parent_region = &subregion_hash[0..subregion_hash.len() - 1];
            let (point, _, _) = geohash::decode(parent_region).unwrap();
            d = SquaredEuclidean::dist(&location, &[point.x, point.y]);
            subregion_hash.pop();
        }
        println!("searching subregion: {}", subregion_hash);

        let mut spatial_index = KdTree::new();
        // ? locate nearby geohashes and populate spatial index
        println!("prefix tree size: {}", self.prefix_tree.len());

        let subtree = self.prefix_tree.clone().split_by_prefix(subregion_hash);

        subtree.iter().for_each(|(ghash, members)| {
            println!("found nearby riders ({}, {:#?})!", ghash, members);
            let (position, _, _) = geohash::decode(&ghash).unwrap();
            members.iter().for_each(|uid| {
                spatial_index.add(&[position.x, position.y], *uid);
            });
        });

        // ? compute nearest neighbors
        println!("spatial index size: {}", spatial_index.size());
        spatial_index
            .within::<SquaredEuclidean>(&location, radius)
            .iter()
            .map(|n| models::Neighbor {
                distance: n.distance * KM_CONVERSION_FACTOR,
                uid: n.item,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{app::AppState, models::PlaceRider};

    #[test]
    fn sandbox() {
        let mut app = AppState::default();

        app.place_rider(PlaceRider {
            uid: 0,
            position: [1.0, 0.0],
        });
        app.place_rider(PlaceRider {
            uid: 1,
            position: [1.0, 1.0],
        });
        app.place_rider(PlaceRider {
            uid: 2,
            position: [0.0, 1.0],
        });
        app.place_rider(PlaceRider {
            uid: 3,
            position: [0.0, 0.0],
        });
        app.place_rider(PlaceRider {
            uid: 4,
            position: [-1.0, 0.0],
        });
        app.place_rider(PlaceRider {
            uid: 5,
            position: [-1.0, -1.0],
        });
        app.place_rider(PlaceRider {
            uid: 6,
            position: [0.0, -1.0],
        });
        app.place_rider(PlaceRider {
            uid: 7,
            position: [0.0, 0.0],
        });
        let center = [0.0, 0.0];
        let radius = 2500.0; // km
        let res = app.search(center, radius);
        println!("riders within {}km from ({:?}): {:#?}", radius, center, res);
    }
}
