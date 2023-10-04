// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_E
// verification-helper: ERROR 0.000001
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default())
}
use circle::Circle;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c1 = Circle::new(cx, cy, r);
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c2 = Circle::new(cx, cy, r);
    let mut ans = c1.cross_point_to_circle(&c2);
    ans.sort();
    let mut v = Vec::new();
    for i in 0..2 {
        v.push(ans[i % ans.len()].x);
        v.push(ans[i % ans.len()].y);
    }
    io.out(v.join(" ").line());

    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "0 0 2
        2 0 2",
        "1.00000000 -1.7320508076 1.00000000 1.7320508076",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "0 0 2
        0 3 1",
        "0.00000000 2.00000000 0.00000000 2.00000000",
    ));
}
