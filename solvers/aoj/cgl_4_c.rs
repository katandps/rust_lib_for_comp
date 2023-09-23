//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C>
use convex_hull::Polygon;
use float_value::FValue;
use plane_float::*;
pub fn solve(
    _n: usize,
    xy: Vec<(f64, f64)>,
    _q: usize,
    p: Vec<(f64, f64, f64, f64)>,
) -> Vec<FValue> {
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
