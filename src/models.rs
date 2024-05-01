use serde::{Deserialize, Serialize};

pub type LatLongCoord = [f64; 2];

#[derive(Deserialize)]
pub struct PlaceRider {
    pub uid: u64,
    pub position: LatLongCoord,
}

#[derive(Serialize)]
pub struct PlaceRiderResponse {
    pub geohash: String,
}

#[derive(Debug, Serialize)]
pub struct Neighbor<A, T> {
    pub distance: A,
    pub uid: T,
}

#[derive(Deserialize)]
pub struct PlaceOrder {
    pub distance: f64,
    pub position: LatLongCoord,
}

#[derive(Serialize)]
pub struct PlaceOrderResponse {
    pub riders: Vec<Neighbor<f64, u64>>,
}

#[derive(Deserialize)]
pub struct RemoveRider {
    pub uid: u64,
}

#[derive(Serialize)]
pub struct RemoveRiderResponse {
    pub removed: bool,
}
