// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/ITP1_7_D
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use library::*;
use matrix::Matrix;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m, l) = io.v3::<usize, usize, usize>();
    let a = io.matrix::<i64>(n, m);
    let b = io.matrix::<i64>(m, l);
    let a = Matrix::build(a).unwrap();
    let b = Matrix::build(b).unwrap();
    let c = (a * b).unwrap();
    io.out(c.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3 2 3
        1 2
        0 3
        4 5
        1 2 1
        0 3 2",
        "1 8 5
        0 9 6
        4 23 14",
    ));
}
