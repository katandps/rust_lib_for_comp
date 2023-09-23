//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_C>
use plane_float::*;
pub fn solve(
    p1: (f64, f64),
    p2: (f64, f64),
    _q: usize,
    xy: &[(f64, f64)],
) -> Vec<ClockwiseDirection> {
    let (p1, p2) = (p1.into(), p2.into());
    xy.iter()
        .cloned()
        .map(|(x, y)| {
            let p3 = (x, y).into();
            ClockwiseDirection::direction(p1, p2, p3)
        })
        .collect()
}

#[test]
fn test() {
    let ans = solve(
        (0.0, 0.0),
        (2.0, 0.0),
        5,
        &vec![
            (-1.0, 1.0),
            (-1.0, -1.0),
            (-1.0, 0.0),
            (0.0, 0.0),
            (3.0, 0.0),
        ],
    );
    assert_eq!(
        ans,
        vec![
            ClockwiseDirection::CounterClockwise,
            ClockwiseDirection::Clockwise,
            ClockwiseDirection::OneLineCAB,
            ClockwiseDirection::OneLineACB,
            ClockwiseDirection::OneLineABC
        ]
    )
}
