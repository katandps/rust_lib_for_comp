// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_B
// verification-helper: ERROR 0.0000000001
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
    let line = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
    for _ in 0..io.v::<usize>() {
        let (x, y) = io.v2::<f64, f64>();
        let p = Vector::new(x, y);
        let result = line.reflection(p);
        io.out(format!("{} {}\n", result.x, result.y))
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "0 0 3 4
        3
        2 5
        1 4
        0 3",
        "4.24 3.32
        3.56 2.08
        2.88 0.84",
    ))
}
#[test]
fn test2() {
    solve(io_debug::IODebug::fvalue_assert(
        "0 0 2 0
        3
        -1 1
        0 1
        1 1",
        "-1 -1
        0 -1
        1 -1",
    ))
}
