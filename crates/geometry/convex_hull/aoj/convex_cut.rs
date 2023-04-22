// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C
// verification-helper: ERROR 0.00000001
use convex_hull::Polygon;
use io_util::*;
use plane_float::{Line, Vector};
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);

    for _ in 0..io.v::<usize>() {
        let (p1x, p1y, p2x, p2y) = io.v4::<f64, f64, f64, f64>();
        let line = Line::new(Vector::new(p1x, p1y), Vector::new(p2x, p2y));
        let ans = polygon.cut(line);
        io.out(ans.area().line());
        assert!(ans.is_convex());
    }
    io.flush();
}
