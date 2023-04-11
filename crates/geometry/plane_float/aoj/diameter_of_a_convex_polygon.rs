// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_B
// verification-helper: ERROR 0.00000001
use io_util::*;
use plane_float::{Point, Polygon};
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);

    io.out(polygon.diameter().line());
    io.flush();
}
