//! # Is-Convex(凸性判定)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl3B;
impl verify::Solver for Cgl3B {
    const PROBLEM_ID: &'static str = "CGL_3_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(n);
        let polygon = Polygon::from(&xy[..]);
        writeln!(write, "{}", usize::from(polygon.is_convex())).ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl3B::assert(
        "4
    0 0
    3 1
    2 3
    0 3",
        "1",
    );
    Cgl3B::assert(
        "5
    0 0
    2 0
    1 1
    2 2
    0 2",
        "0",
    )
}
