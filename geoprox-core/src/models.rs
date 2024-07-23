use serde::Serialize;

pub type ResourceIdentifier = u64;

pub type LatLngCoord = [f64; 2];

#[derive(Debug, Serialize)]
pub struct Neighbor<D = f64> {
    pub distance: D,
    pub resource: String,
}