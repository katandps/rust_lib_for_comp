//! # Incircle of a Triangle(内接円)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl7B;
impl verify::Solver for Cgl7B {
    const PROBLEM_ID: &'static str = "CGL_7_B";
    const EPSILON: Option<f64> = Some(1e-6);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let xy = reader.vec2::<f64, f64>(3);
        let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
        let tri = Triangle::new(p[0], p[1], p[2]);
        let circle = tri.inner_circle().unwrap();
        writeln!(
            write,
            "{} {} {}",
            circle.center.x, circle.center.y, circle.radius
        )
        .ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl7B::assert(
        "1 -2
    3 2
    -2 0",
        "0.53907943898209422325 -0.26437392711448356856 1.18845545916395465278",
    );
    Cgl7B::assert(
        "0 3
    4 0
    0 0",
        "1 1 1",
    )
}
