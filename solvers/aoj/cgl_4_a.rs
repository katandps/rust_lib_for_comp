//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A>
use convex_hull::*;
use float_value::FValue;
use plane_float::*;
pub fn solve(_n: usize, xy: &[(f64, f64)]) -> Vec<(FValue, FValue)> {
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
    ans.nodes.iter().map(|v| (v.x, v.y)).collect()
}

#[test]
fn test() {
    let n = 7;
    let xy = vec![
        (2.0, 1.0),
        (0.0, 0.0),
        (1.0, 2.0),
        (2.0, 2.0),
        (4.0, 2.0),
        (1.0, 3.0),
        (3.0, 3.0),
    ];
    let ans = solve(n, &xy);
    assert_eq!(5, ans.len());
    assert_eq!(
        ans,
        vec![(0, 0), (2, 1), (4, 2), (3, 3), (1, 3)]
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect::<Vec<(FValue, FValue)>>()
    )
}
