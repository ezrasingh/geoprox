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
    pub fn place_rider(&mut self, payload: models::PlaceRider) -> models::PlaceRiderResponse {
        let ghash = geohash::encode(
            geohash::Coord {
                x: payload.position[0],
                y: payload.position[1],
            },
            self.precision,
        )
        .unwrap();
        match self.position_map.insert(payload.uid, ghash.clone()) {
            Some(old_ghash) => {
                // ? remove rider from previous region
                match self.prefix_tree.get_mut(old_ghash.clone()) {
                    Some(prev_members) => {
                        prev_members.remove(&payload.uid);
                    }
                    None => {
                        self.prefix_tree.remove(old_ghash);
                    }
                }
            }
            None => {}
        }
        // ? insert current region in prefix tree
        match self.prefix_tree.get_mut(ghash.clone()) {
            Some(members) => {
                members.insert(payload.uid);
            }
            None => {
                let mut members: HashSet<u64> = HashSet::new();
                members.insert(payload.uid);
                self.prefix_tree.insert(ghash.clone(), members);
            }
        };
        println!("placed rider: ({}, {})", payload.uid, ghash.clone());
        models::PlaceRiderResponse { geohash: ghash }
    }

    pub fn place_order(self, payload: models::PlaceOrder) -> models::PlaceOrderResponse {
        let neighbors = self.search(payload.position, payload.distance);
        models::PlaceOrderResponse { riders: neighbors }
    }

    pub fn remove_rider(&mut self, payload: models::RemoveRider) -> models::RemoveRiderResponse {
        self.position_map.remove(&payload.uid);
        models::RemoveRiderResponse { removed: true }
    }

    fn search(
        self,
        location: models::LatLongCoord,
        radius: f64,
    ) -> Vec<models::Neighbor<f64, u64>> {
        let within = radius / KM_CONVERSION_FACTOR;
        let mut subregion_hash = geohash::encode(location.into(), self.precision).unwrap();
        let mut d = 0.0;
        while d < within {
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
            .within::<SquaredEuclidean>(&location, within)
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
        let radius = 1000.0; // km
        let res = app.search(center, radius);
        println!("riders within {} from ({:?}): {:#?}", radius, center, res);
    }
}
