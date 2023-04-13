//! # 最近点対
use plane_float::Point;
pub struct ClosestPair(Vec<Point>);
impl ClosestPair {
    pub fn closest_pair(mut points: Vec<Point>) -> (f64, Point, Point) {
        assert!(points.len() >= 2);
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut n = points.len();
        let m = n / 2;

        (0.0, points[0], points[1])
    }
}
