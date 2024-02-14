//! # Cross Point(交点)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl2C;
impl verify::Solver for Cgl2C {
    const PROBLEM_ID: &'static str = "CGL_2_C";
    const EPSILON: Option<f64> = Some(1e-8);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
            let (x3, y3, x4, y4) = reader.v4::<f64, f64, f64, f64>();
            if let Some(result) = Segment::cross_point(
                Segment::new((x1, y1).into(), (x2, y2).into()),
                Segment::new((x3, y3).into(), (x4, y4).into()),
            ) {
                writeln!(write, "{} {}", result.x, result.y).ok();
            } else {
                panic!("line is parallel")
            }
        }
        write.flush().ok();
    }
}
