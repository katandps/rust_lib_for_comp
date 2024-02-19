//! # Reflection(反射)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl1B;
impl verify::Solver for Cgl1B {
    const PROBLEM_ID: &'static str = "CGL_1_B";
    const EPSILON: Option<f64> = Some(1e-8);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(
        read: impl rust_lib_for_comp::prelude::Read,
        mut write: impl rust_lib_for_comp::prelude::Write,
    ) {
        let mut reader = ReadHelper::new(read);
        let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
        let line = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
        for _ in 0..reader.v::<usize>() {
            let (x, y) = reader.v2::<f64, f64>();
            let p = Vector::new(x, y);
            let result = line.reflection(p);
            writeln!(write, "{} {}", result.x, result.y).ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl1B::assert(
        "0 0 3 4
    3
    2 5
    1 4
    0 3",
        "4.2400000000 3.3200000000
        3.5600000000 2.0800000000
        2.8800000000 0.8400000000",
    );
    Cgl1B::assert(
        "0 0 2 0
        3
        -1 1
        0 1
        1 1",
        "-1.0000000000 -1.0000000000
        0.0000000000 -1.0000000000
        1.0000000000 -1.0000000000",
    );
}
