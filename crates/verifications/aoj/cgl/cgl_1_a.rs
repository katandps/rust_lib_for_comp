//! # Projection(射影)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl1A;
impl verify::Solver for Cgl1A {
    const PROBLEM_ID: &'static str = "CGL_1_A";
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
            let result = line.projection(p);
            writeln!(write, "{} {}", result.x, result.y).ok();
        }
        write.flush().ok();
    }
}
