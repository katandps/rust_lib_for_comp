//! # Intersection(交差判定)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl2B;
impl verify::Solver for Cgl2B {
    const PROBLEM_ID: &'static str = "CGL_2_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
            let (x3, y3, x4, y4) = reader.v4::<f64, f64, f64, f64>();
            let (s1, s2) = (
                Segment::new(Vector::new(x1, y1), Vector::new(x2, y2)),
                Segment::new(Vector::new(x3, y3), Vector::new(x4, y4)),
            );
            writeln!(write, "{}", usize::from(Segment::is_intersect(s1, s2))).ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl2B::assert(
        "3
    0 0 3 0 1 1 2 -1
    0 0 3 0 3 1 3 -1
    0 0 3 0 3 -2 5 0",
        "1
        1
        0",
    )
}
