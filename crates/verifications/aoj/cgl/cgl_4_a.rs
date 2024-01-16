//! # Convex-Hull(凸包)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl4A;
impl verify::Solver for Cgl4A {
    const PROBLEM_ID: &'static str = "CGL_4_A";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(n);
        let points = xy
            .iter()
            .cloned()
            .map(|(x, y)| Vector::new(x, y))
            .collect::<Vec<_>>();
        let convex_hull = Polygon::convex_hull(points, true);
        let mut poly = Polygon::new(convex_hull.nodes.into_iter().map(Vector::swap).collect());
        // yについて正規化
        poly.normalize();
        let ans = Polygon::new(poly.nodes.into_iter().map(Vector::swap).collect());
        writeln!(write, "{}", ans.nodes.len()).ok();
        for v in ans.nodes {
            writeln!(write, "{} {}", v.x, v.y).ok();
        }
        write.flush().ok();
    }
}
