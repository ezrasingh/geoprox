use serde::Serialize;

/// Object indentifier
pub type ObjectIdentifier = u64;

/// Latitude/longitude coordinate pair
pub type LatLngCoord = [f64; 2];

/// Nearby object
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Neighbor {
    /// Distance in kilometers
    pub distance: f64,
    /// Object key
    pub key: String,
}
