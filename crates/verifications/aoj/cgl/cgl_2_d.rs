//! # Distance(距離)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl2D;
impl verify::Solver for Cgl2D {
    const PROBLEM_ID: &'static str = "CGL_2_D";
    const EPSILON: Option<f64> = Some(1e-8);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let p0 = reader.v2::<f64, f64>();
            let p1 = reader.v2::<f64, f64>();
            let p2 = reader.v2::<f64, f64>();
            let p3 = reader.v2::<f64, f64>();
            let ans = Segment::distance(
                Segment::new(p0.into(), p1.into()),
                Segment::new(p2.into(), p3.into()),
            );
            writeln!(write, "{ans}").ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl2D::assert(
        "3
    0 0 1 0 0 1 1 1
    0 0 1 0 2 1 1 2
    -1 0 1 0 0 1 0 -1",
        "1.0000000000
        1.4142135624
        0.0000000000",
    )
}
