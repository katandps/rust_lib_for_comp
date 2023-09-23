//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_F>
use circle::*;
use float_value::FValue;
pub fn solve(p: (f64, f64), c: (f64, f64), r: f64) -> ((FValue, FValue), (FValue, FValue)) {
    let p = p.into();
    let c = Circle::new(c.0, c.1, r);

    let mut ans = c.tangent(p);
    ans.sort();
    (ans[0].into(), ans[1].into())
}

#[test]
fn test() {
    assert_eq!(
        solve((0.0, 0.0), (2.0, 2.0), 2.0),
        ((0.0.into(), 2.0.into()), (2.0.into(), 0.0.into()))
    );
    assert_eq!(
        solve((-3.0, 0.0), (2.0, 2.0), 2.0),
        (
            (0.6206896552.into(), 3.4482758621.into()),
            (2.0.into(), 0.0.into())
        )
    )
}
