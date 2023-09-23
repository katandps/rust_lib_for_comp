//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_C>
use circle::*;
use plane_float::*;
pub fn solve(xy: &[(f64, f64)]) -> (f64, f64, f64) {
    let p: Vec<_> = xy.iter().cloned().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.circumscribed_circle().unwrap();
    (circle.center.x.0, circle.center.y.0, circle.radius.0)
}

#[test]
fn test() {
    let ans = solve(&vec![(1.0, -2.0), (3.0, 2.0), (-2.0, 0.0)]);
    assert_eq!(
        ans,
        (0.625.into(), 0.6875.into(), 2.71353666826155124291.into())
    )
}
