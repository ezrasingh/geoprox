use kiddo::distance_metric::DistanceMetric;
use haversine::Units;

pub struct HaversineMetric;

impl DistanceMetric<f64, 2> for HaversineMetric {
    /// Computes the Haversine distance between two 2D points given latitude and longitude.
    fn dist(a: &[f64; 2], b: &[f64; 2]) -> f64 {
        haversine::distance(
            haversine::Location{ latitude: a[1], longitude: a[0] },
            haversine::Location{ latitude: b[1], longitude: b[0] },
            Units::Kilometers
        )
    }

    /// Computes the absolute difference between two values along a single axis (latitude or longitude).
    fn dist1(a: f64, b: f64) -> f64 {
        (a - b).abs()
    }
}