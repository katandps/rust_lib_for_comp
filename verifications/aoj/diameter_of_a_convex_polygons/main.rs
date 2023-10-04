// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_B
// verification-helper: ERROR 0.00000001
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use convex_hull::*;
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);

    io.out(polygon.diameter().line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "3
        0.0 0.0
        4.0 0.0
        2.0 2.0",
        "4.00",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "4
        0.0 0.0
        1.0 0.0
        1.0 1.0
        0.0 1.0",
        "1.414213562373",
    ));
}
