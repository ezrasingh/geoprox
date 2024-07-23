use serde::{Deserialize, Serialize};
use geoprox_core::models::{LatLngCoord,Neighbor,ResourceIdentifier};

#[derive(Serialize)]
pub struct CreateIndexResponse {
    pub created: bool,
    pub existed: bool,
}


#[derive(Deserialize)]
pub struct InsertResource {
    pub key: String,
    pub position: LatLngCoord,
}

#[derive(Serialize)]
pub struct InsertResourceResponse {
    pub key: String,
    pub geohash: String,
}

#[derive(Serialize)]
pub struct DropIndexResponse {
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct QueryRange {
    pub origin: LatLngCoord,
    pub range: u16,
}

#[derive(Serialize)]
pub struct QueryRangeResponse {
    pub found: Vec<Neighbor>,
}