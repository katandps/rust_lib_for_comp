//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A>
use convex_hull::Polygon;
use float_value::FValue;

pub fn solve(_n: usize, xy: Vec<(f64, f64)>) -> FValue {
    let polygon = Polygon::from(&xy[..]);
    polygon.area()
}

#[test]
fn test() {
    let n = 4;
    let xy = vec![(0.0, 0.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    assert_eq!(1.5, solve(n, xy).0)
}
