// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A
// verification-helper: ERROR 0.1
use convex_hull::Polygon;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    io.out(solve(n, xy).line());
    io.flush();
}

fn solve(_n: usize, xy: Vec<(f64, f64)>) -> float_value::FValue {
    let polygon = Polygon::from(&xy[..]);
    polygon.area()
}

#[test]
fn test() {
    let n = 4;
    let xy = vec![(0.0, 0.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    assert_eq!(1.5, solve(n, xy).0)
}
