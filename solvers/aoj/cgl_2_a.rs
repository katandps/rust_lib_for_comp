//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_2_A>
use plane_float::*;

pub fn solve(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> usize {
    let (l1, l2) = (
        Line::new(p0.into(), p1.into()),
        Line::new(p2.into(), p3.into()),
    );
    if Line::is_parallel(l1, l2) {
        2
    } else if Line::is_orthogonal(l1, l2) {
        1
    } else {
        0
    }
}

#[test]
fn test() {
    assert_eq!(solve((0.0, 0.0), (3.0, 0.0), (0.0, 2.0), (3.0, 2.0)), 2);
    assert_eq!(solve((0.0, 0.0), (3.0, 0.0), (1.0, 1.0), (1.0, 4.0)), 1);
    assert_eq!(solve((0.0, 0.0), (3.0, 0.0), (1.0, 1.0), (2.0, 2.0)), 0);
}
