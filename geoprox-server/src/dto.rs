use serde::{Deserialize, Serialize};
use geoprox_core::models::{LatLngCoord,Neighbor};

#[derive(Serialize)]
pub struct DecodeGeohashResponse {
    pub lat: f64,
    pub lat_error: f64,
    pub lng: f64,
    pub lng_error: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EncodeLatLng {
    pub lat: f64,
    pub lng: f64,
    pub depth: usize,
}

#[derive(Serialize)]
pub struct EncodeLatLngResponse {
    pub geohash: String,
}

#[derive(Serialize)]
pub struct GeohashNeighborsResponse {
    pub sw: String,
    pub s: String,
    pub se: String,
    pub w: String,
    pub e: String,
    pub nw: String,
    pub n: String,
    pub ne: String,
}

impl From<geoprox_core::geohash::Neighbors> for GeohashNeighborsResponse{
    fn from(value: geoprox_core::geohash::Neighbors) -> Self {
        Self {
            sw: value.sw,
            s: value.s,
            se: value.se,
            w: value.w,
            e: value.e,
            nw: value.nw,
            n: value.n,
            ne: value.ne,
        }
    }
}

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
    pub lat: f64,
    pub lng: f64,
    pub range: u16,
}

#[derive(Serialize)]
pub struct QueryRangeResponse {
    pub found: Vec<Neighbor>,
}