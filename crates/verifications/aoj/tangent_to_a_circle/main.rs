// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_F
// verification-helper: ERROR 0.00000001
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use circle::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let p = io.v2::<f64, f64>();
    let c = io.v2::<f64, f64>();
    let r = io.v::<f64>();
    let p = p.into();
    let c = Circle::new(c.0, c.1, r);

    let mut ans = c.tangent(p);
    ans.sort();
    io.out(format!("{} {}", ans[0].x, ans[0].y).line());
    io.out(format!("{} {}", ans[1].x, ans[1].y).line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "0 0
        2 2 2",
        "0.0000000000 2.0000000000
        2.0000000000 0.0000000000",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "-3 0
        2 2 2",
        "0.6206896552 3.4482758621
        2.0000000000 0.0000000000",
    ))
}
