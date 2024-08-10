use haversine_rs::point::Point;
use haversine_rs::units::Unit;
use kiddo::distance_metric::DistanceMetric;

pub struct HaversineDistance;

impl DistanceMetric<f64, 2> for HaversineDistance {
    /// Computes the Haversine distance between two 2D points given latitude and longitude.
    #[inline]
    fn dist(a: &[f64; 2], b: &[f64; 2]) -> f64 {
        haversine_rs::distance(
            Point {
                latitude: a[0],
                longitude: a[1],
            },
            Point {
                latitude: b[0],
                longitude: b[1],
            },
            Unit::Kilometers,
        )
    }

    /// Computes the absolute difference between two values along a single axis (latitude or longitude).
    #[inline]
    fn dist1(a: f64, b: f64) -> f64 {
        (a - b).abs()
    }
}
