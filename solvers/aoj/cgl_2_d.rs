//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_D>
use float_value::FValue;
use plane_float::*;

pub fn solve(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> FValue {
    Segment::distance(
        Segment::new(p0.into(), p1.into()),
        Segment::new(p2.into(), p3.into()),
    )
}

#[test]
fn test() {
    assert_eq!(
        solve((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)),
        1.0.into()
    );
    assert_eq!(
        solve((0.0, 0.0), (1.0, 0.0), (2.0, 1.0), (1.0, 2.0)),
        1.4142135624.into()
    );
    assert_eq!(
        solve((-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0)),
        0.0.into()
    );
}
