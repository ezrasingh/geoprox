use serde::Serialize;

pub type UserIdentifier = u64;

pub type LatLongCoord = [f64; 2];

#[derive(Debug, Serialize)]
pub struct Neighbor<D> {
    pub distance: D,
    pub uid: UserIdentifier,
}