//! # Area(面積)

use rust_lib_for_comp::geometry::convex_hull::Polygon;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl3A;
impl verify::Solver for Cgl3A {
    const PROBLEM_ID: &'static str = "CGL_3_A";
    const EPSILON: Option<f64> = Some(1e-6);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(n);
        let polygon = Polygon::from(&xy[..]);
        writeln!(write, "{}", polygon.area()).ok();
        write.flush().ok();
    }
}
