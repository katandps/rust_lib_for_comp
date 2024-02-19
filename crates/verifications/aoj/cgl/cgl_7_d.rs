//! # Cross Points of a Circe and a Line(円と直線の交点)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl7D;
impl verify::Solver for Cgl7D {
    const PROBLEM_ID: &'static str = "CGL_7_D";
    const EPSILON: Option<f64> = Some(1e-8);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (cx, cy, r) = reader.v3::<f64, f64, f64>();
        let c = Circle::new(cx, cy, r);
        for _ in 0..reader.v::<usize>() {
            let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
            let l = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
            let mut ans = c.cross_point_to_line(&l);
            ans.sort();
            let mut v = Vec::new();
            for i in 0..2 {
                v.push(ans[i % ans.len()].x);
                v.push(ans[i % ans.len()].y);
            }
            writeln!(write, "{}", v.iter().join(" ")).ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Cgl7D::assert(
        "2 1 1
    2
    0 1 4 1
    3 0 3 3",
        "1 1 3 1
        3 1 3 1",
    )
}
