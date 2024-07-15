use serde::{Deserialize, Serialize};
use geoprox_core::models::{LatLongCoord,Neighbor,UserIdentifier};

#[derive(Deserialize)]
pub struct PlaceRider {
    pub uid: UserIdentifier,
    pub position: LatLongCoord,
}

#[derive(Serialize)]
pub struct PlaceRiderResponse {
    pub geohash: String,
}


#[derive(Deserialize)]
pub struct PlaceOrder {
    pub distance: f64,
    pub position: LatLongCoord,
}

#[derive(Serialize)]
pub struct PlaceOrderResponse {
    pub riders: Vec<Neighbor<f64>>,
}

#[derive(Deserialize)]
pub struct RemoveRider {
    pub uid: UserIdentifier,
}

#[derive(Serialize)]
pub struct RemoveRiderResponse {
    pub removed: bool,
}
