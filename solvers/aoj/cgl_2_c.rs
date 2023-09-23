//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C>
use float_value::*;
use plane_float::*;

pub fn solve(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> (FValue, FValue) {
    if let Some(result) = Segment::cross_point(
        Segment::new(p0.into(), p1.into()),
        Segment::new(p2.into(), p3.into()),
    ) {
        (result.x, result.y)
    } else {
        panic!("line is parallel")
    }
}

#[test]
fn test() {
    assert_eq!(
        solve((0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (1.0, -1.0)),
        (1.0.into(), 0.0.into())
    );
    assert_eq!(
        solve((0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0)),
        (0.5.into(), 0.5.into())
    );
    assert_eq!(
        solve((0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)),
        (0.5.into(), 0.5.into())
    );
}
