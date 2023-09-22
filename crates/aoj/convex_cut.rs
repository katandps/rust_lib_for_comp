// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C
// verification-helper: ERROR 0.00000001
use convex_hull::Polygon;
use float_value::FValue;
use io_util::*;
use plane_float::{Line, Vector};
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let mut p = Vec::new();
    let q = io.v::<usize>();
    for _ in 0..q {
        p.push(io.v4::<f64, f64, f64, f64>());
    }
    for ans in solve(n, xy, q, p) {
        io.out(ans.line())
    }
    io.flush();
}

fn solve(_n: usize, xy: Vec<(f64, f64)>, _q: usize, p: Vec<(f64, f64, f64, f64)>) -> Vec<FValue> {
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);
    let mut ret = Vec::new();
    for (p1x, p1y, p2x, p2y) in p {
        let line = Line::new(Vector::new(p1x, p1y), Vector::new(p2x, p2y));
        let ans = polygon.cut(line);
        assert!(ans.is_convex());
        ret.push(ans.area());
    }
    ret
}

#[test]
fn test() {
    let n = 4;
    let xy = vec![(1.0, 1.0), (4.0, 1.0), (4.0, 3.0), (1.0, 3.0)];
    let q = 2;
    let p = vec![(2.0, 0.0, 2.0, 4.0), (2.0, 4.0, 2.0, 0.0)];
    let ans = solve(n, xy, q, p);
    assert_eq!(ans, vec![2.0.into(), 4.0.into()]);
}
