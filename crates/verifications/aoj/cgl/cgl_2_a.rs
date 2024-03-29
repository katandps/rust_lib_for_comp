//! # Parallel/Orthogonal(平行・垂直)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};
#[derive(AizuOnlineJudge)]
pub struct Cgl2A;
impl verify::Solver for Cgl2A {
    const PROBLEM_ID: &'static str = "CGL_2_A";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let p0 = reader.v2::<f64, f64>();
            let p1 = reader.v2::<f64, f64>();
            let p2 = reader.v2::<f64, f64>();
            let p3 = reader.v2::<f64, f64>();
            let (l1, l2) = (
                Line::new(p0.into(), p1.into()),
                Line::new(p2.into(), p3.into()),
            );
            let ans = if Line::is_parallel(l1, l2) {
                2
            } else if Line::is_orthogonal(l1, l2) {
                1
            } else {
                0
            };
            writeln!(write, "{ans}").ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl2A::assert(
        "3
    0 0 3 0 0 2 3 2
    0 0 3 0 1 1 1 4
    0 0 3 0 1 1 2 2",
        "2
        1
        0",
    )
}
