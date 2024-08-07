use haversine::Units;
use kiddo::distance_metric::DistanceMetric;

pub struct HaversineDistance;

impl DistanceMetric<f64, 2> for HaversineDistance {
    /// Computes the Haversine distance between two 2D points given latitude and longitude.
    fn dist(a: &[f64; 2], b: &[f64; 2]) -> f64 {
        haversine::distance(
            haversine::Location {
                latitude: a[0],
                longitude: a[1],
            },
            haversine::Location {
                latitude: b[0],
                longitude: b[1],
            },
            Units::Kilometers,
        )
    }

    /// Computes the absolute difference between two values along a single axis (latitude or longitude).
    fn dist1(a: f64, b: f64) -> f64 {
        (a - b).abs()
    }
}
