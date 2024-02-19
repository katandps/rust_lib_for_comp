//! # Closest Pair(最近点対)

use rust_lib_for_comp::geometry::closest_pair::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl5A;
impl verify::Solver for Cgl5A {
    const PROBLEM_ID: &'static str = "CGL_5_A";
    const EPSILON: Option<f64> = Some(1e-6);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(n);
        let points = xy
            .into_iter()
            .map(|(x, y)| Vector::new(x, y))
            .collect::<Vec<_>>();
        let (dist, _pair) = ClosestPair::closest_pair(points);
        writeln!(write, "{dist}").ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl5A::assert(
        "2
    0.0 0.0
    1.0 0.0",
        "1.0",
    );
    Cgl5A::assert(
        "3
    0.0 0.0
    2.0 0.0
    1.0 1.0",
        "1.41421356237",
    )
}
