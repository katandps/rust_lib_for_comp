// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_B
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use convex_hull::Polygon;
use io_util::*;
use string_util::*;
pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    io.out(usize::from(polygon.is_convex()).line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4
        0 0
        3 1
        2 3
        0 3",
        "1",
    ))
}
#[test]
fn test2() {
    solve(io_debug::IODebug::static_assert(
        "5
        0 0
        2 0
        1 1
        2 2
        0 2",
        "0",
    ))
}
