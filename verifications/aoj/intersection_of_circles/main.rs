// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_A
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use circle::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (x1, y1, r1) = io.v3::<f64, f64, f64>();
    let (x2, y2, r2) = io.v3::<f64, f64, f64>();
    let c1 = Circle::new(x1, y1, r1);
    let c2 = Circle::new(x2, y2, r2);
    io.out(
        match CircleIntersection::intersect(&c1, &c2) {
            CircleIntersection::NotCross => 4,
            CircleIntersection::Circumscribed => 3,
            CircleIntersection::Intersect => 2,
            CircleIntersection::Inscribed => 1,
            CircleIntersection::Included => 0,
        }
        .line(),
    );
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "1 1 1
        6 2 2",
        "4",
    ));
    solve(io_debug::IODebug::static_assert(
        "1 2 1
        4 2 2",
        "3",
    ));
    solve(io_debug::IODebug::static_assert(
        "1 2 1
        3 2 2",
        "2",
    ));
    solve(io_debug::IODebug::static_assert(
        "0 0 1
        1 0 2",
        "1",
    ));
    solve(io_debug::IODebug::static_assert(
        "0 0 1
        0 0 2",
        "0",
    ));
}
