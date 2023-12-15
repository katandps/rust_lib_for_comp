//! # 最近点対
use crate::prelude::*;
use float_value::FValue;
use min_max_macro::{chmin, min};
use plane_float::Vector;

#[codesnip::entry("closest-pair", doc_hidden)]
pub struct ClosestPair;
#[codesnip::entry("closest-pair", doc_hidden)]
impl ClosestPair {
    const INF: f64 = 1e30;
    pub fn closest_pair(mut points: Vec<Vector>) -> (FValue, Option<(Vector, Vector)>) {
        assert!(points.len() >= 2);
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (d2, pair) = Self::calc(&mut points);
        (d2.sqrt(), pair)
    }

    // 距離の二乗と、その値を得るペアを得る
    fn calc(points: &mut [Vector]) -> (FValue, Option<(Vector, Vector)>) {
        let n = points.len();
        if n <= 1 {
            return (Self::INF.into(), None);
        }
        let mid = n / 2;
        let mx = points[mid].x;

        let (d2_left, first_pair) = Self::calc(&mut points[0..mid]);
        let (d2_right, second_pair) = Self::calc(&mut points[mid..]);
        let (mut d2, mut pair) = (d2_left, first_pair);
        if chmin!(d2, d2_right) {
            pair = second_pair;
        }
        points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        let mut nears = VecDeque::new();
        for &mut p in points {
            let dx = p.x - mx;
            if dx * dx >= d2 {
                continue;
            }
            for j in (0..nears.len()).rev() {
                let a: Vector = nears[j] - p;
                let dy2 = a.y * a.y;
                if dy2 < d2 {
                    let dx2 = a.x * a.x;
                    if chmin!(d2, dx2 + dy2) {
                        pair = Some((nears[j], p));
                    }
                } else {
                    for _ in 0..j {
                        nears.pop_front();
                    }
                    break;
                }
            }
            nears.push_back(p)
        }
        (d2, pair)
    }
}
