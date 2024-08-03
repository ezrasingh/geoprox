use serde::Serialize;

/// Resource key indentifier
pub type ResourceIdentifier = u64;

/// Latitude/longitude pair
pub type LatLngCoord = [f64; 2];

/// Nearby resource key
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize)]
pub struct Neighbor {
    /// Distance in kilometers
    pub distance: f64,
    /// Resource key
    pub key: String,
}
