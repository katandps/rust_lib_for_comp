//! # Circumscribed Circle of a Triangle(外接円)

use rust_lib_for_comp::geometry::{circle::*, plane_float::*};
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl7C;
impl verify::Solver for Cgl7C {
    const PROBLEM_ID: &'static str = "CGL_7_C";
    const EPSILON: Option<f64> = Some(1e-6);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let xy = reader.vec2::<f64, f64>(3);
        let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
        let tri = Triangle::new(p[0], p[1], p[2]);
        let circle = tri.circumscribed_circle().unwrap();
        writeln!(
            write,
            "{} {} {}",
            circle.center.x.0, circle.center.y.0, circle.radius.0
        )
        .ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl7C::assert(
        "1 -2
    3 2
    -2 0",
        "0.62500000000000000000 0.68750000000000000000 2.71353666826155124291",
    );
    Cgl7C::assert(
        "0 3
    4 0
    0 0",
        "2 1.5 2.5",
    )
}
