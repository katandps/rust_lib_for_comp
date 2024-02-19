//! # Polygon-Point-Containment(多角形-点の包含)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl3C;
impl verify::Solver for Cgl3C {
    const PROBLEM_ID: &'static str = "CGL_3_C";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(n);
        let polygon = Polygon::from(&xy[..]);
        for _ in 0..reader.v::<usize>() {
            let p = Vector::from(reader.v2::<f64, f64>());
            writeln!(
                write,
                "{}",
                match polygon.include(p) {
                    Including::Inside => 2,
                    Including::OnLine => 1,
                    Including::Outside => 0,
                },
            )
            .ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl3C::assert(
        "4
    0 0
    3 1
    2 3
    0 3
    3
    2 1
    0 2
    3 2",
        "2
        1
        0",
    )
}
